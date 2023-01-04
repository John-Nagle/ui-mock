// # uiinfo.rs  --  application level data for the user interface.
//
//  Animats
//  November 2022
//
use super::dialogs::dialoglogin::LoginParams;
use anyhow::{anyhow, Context, Error};
use libui::load_image;
use libui::{t, AppState, CommonState, GuiState};
use rend3::Renderer;
use rend3_egui::EguiRenderRoutine;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

/// User events sent to the main event loop.
//  These are specific to the application. libui has a few more.
#[derive(Debug)]
pub enum GuiEvent {
    Startup,                     // back to startup state
    OpenReplay(Option<PathBuf>), // open a replay file
    SaveReplay(PathBuf),         // save into a replay file
    LoginTo(GridSelectParams),   // ask for login params
    LoginStart(LoginParams),     // start the login process
                                 ////Login(ConnectInfo),                         // login dialog result
}

/// GUI states.
//  The main states of the system.
//  This is a state machine
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SystemMode {
    Startup, // idle, waiting for grid selection
    // -> Login, Replay. Exit
    Login, // login dialog is up.
    // -> Connected, Start
    Connecting, // Connecting to server
    // -> Connected, Start
    Connected, // Fully connected, all menus live
    // -> Shutdown
    Replay, // in replay mode, some menus live
    // -> Shutdown
    Shutdown, // shutting down and cleaning up
              // -> Exit, Start
}

/// AppAssets -- compile time assets of the UI.
pub struct UiAppAssets {
    pub move_arrows_icon: egui::TextureId,
    pub rot_arrows_icon: egui::TextureId,
    pub pressed_button_icon: egui::TextureId,
}

/// Part of AppUi
pub struct UiData {
    //  These keep reference-counted Rend3 objects alive.
    pub _object_handle: rend3::types::ObjectHandle,
    pub _material_handle: rend3::types::MaterialHandle,
    pub _directional_handle: rend3::types::DirectionalLightHandle,

    pub egui_routine: rend3_egui::EguiRenderRoutine,
    pub start_time: instant::Instant,
    pub quit: bool, // global quit flag

    //  The 2D GUI
    pub gui_state: GuiState<UiInfo>, // state of the GUI

    //  Assets of the application.
    pub ui_app_assets: UiAppAssets,
}

impl UiData {}

/// Generic parameter to GuiState, containing app-specific info.
//  Data passed through GuiState, but not interpreted by it.
#[derive(Debug)]
pub struct UiInfo {
    //  Primary system mode
    system_mode: SystemMode, // primary operating mode
    //  Selected grid
    pub selected_grid: Option<GridSelectParams>, // params of selected grid, if any
    //  All the grids, read-only and shareable
    pub grid_select_params: Rc<Vec<GridSelectParams>>,
}

impl UiInfo {
    /// Usual new
    pub fn new(grid_select_params: Vec<GridSelectParams>) -> Self {
        Self {
            system_mode: SystemMode::Startup, // start in Startup mode
            selected_grid: None,              // with no grid
            grid_select_params: Rc::new(grid_select_params), // all possible grids
        }
    }
    /// Change main system mode. Login, select grid, run, etc.
    pub fn change_mode(&mut self, new_mode: SystemMode) {
        log::info!(
            "System state change: {:?} -> {:?}",
            self.system_mode,
            new_mode
        );
        println!(
            "System state change: {:?} -> {:?}",
            self.system_mode, new_mode
        ); // ***TEMP***
        self.system_mode = new_mode; // do state change
    }

    /// Access
    pub fn get_mode(&self) -> SystemMode {
        self.system_mode
    }
}

impl AppState for UiInfo {}

/// App-specific utility functions.
/// Pick replay file, async form
#[cfg(feature = "replay")]
pub fn pick_replay_file_async(state: &mut CommonState, window: &winit::window::Window) {
    fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
        // this is stupid... use any executor of your choice instead
        std::thread::spawn(move || futures::executor::block_on(f));
    }

    let channel = state.get_send_channel().clone(); // save send channel
                                                    //  Pop up the file dialog
    let task = rfd::AsyncFileDialog::new()
        .set_title(t!("title.open_replay", state.get_lang()))
        .add_filter("json", &["json"])
        .set_parent(window)
        .pick_file();
    // Await somewhere else
    execute(async move {
        let file = task.await; // wait for dialog completion
        let replay_path_opt = if let Some(file) = file {
            // If you are on native platform you can just get the path
            println!("{:?}", file.path());

            // If you care about wasm support you just read() the file
            ////file.read().await;
            log::warn!("File picked: {:?}", file.path());
            Some(file.path().to_path_buf())
        } else {
            None
        };
        //  Send dialog result to the main event loop for action.
        let _ = CommonState::send_gui_event_on_channel(
            &channel,
            Box::new(GuiEvent::OpenReplay(replay_path_opt)),
        ); // if we can't send, we must be shutting down
    });
}

/// GridSelectParams file contents.
#[derive(Debug, Clone, Deserialize)]
pub struct GridSelectParamsData {
    pub metaverse: String,         // Second Life, OsGrid, etc.
    pub grid: String,              // agni, etc.
    pub picture_bar: String,       // local file name in images directory
    pub home_url: String,          // home page for site
    pub join_url: Option<String>,  // How to join
    pub login_url: Option<String>, // if none, this is a replay
    pub comment: Option<String>,   // to allow a comment in the source JSON file
}

/// This describes the format of the grids.json file for serde deserialization.
#[derive(Debug, Clone, Deserialize)]
struct GridSelectParamsDataJson {
    pub grids: Vec<GridSelectParamsData>,
}

#[derive(Debug, Clone)]
pub struct GridSelectParams {
    pub data: GridSelectParamsData,   // as read from JSON
    pub picture_bar: egui::TextureId, // texture has been loaded and is ready to go
}

impl GridSelectParams {
    /// Read the JSON grid select params file tnto a GridSelectParams structure.
    pub fn read_grid_select_params(
        filename: &PathBuf,
        asset_dir: &Path,
        egui_routine: &mut EguiRenderRoutine,
        renderer: &Arc<Renderer>,
    ) -> Result<Vec<GridSelectParams>, Error> {
        //  Read grid_select file
        let mut grid_file = asset_dir.to_path_buf();
        grid_file.push(filename);
        let file = File::open(grid_file).with_context(|| {
            anyhow!(
                "Failed to open the grid select params config file: {:?}",
                filename
            )
        })?;
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .context("Failed to read the grid select params config.")?;
        let grids_data: GridSelectParamsDataJson = serde_json::from_str(&content)
            .context("Failed to parse grid select params config file.")?;
        let mut params = Vec::new();
        for data in grids_data.grids {
            let mut image_file_name = asset_dir.to_path_buf(); // build file name of image
            image_file_name.push(&data.picture_bar);
            println!(
                "Metaverse: {} Grid: {} Picture bar image file: {:?}",
                data.metaverse, data.grid, image_file_name
            ); // ***TEMP***
            let image = image::io::Reader::open(&image_file_name)
                .with_context(|| {
                    format!(
                        "Unable to open image file {:?} for grid menu",
                        image_file_name
                    )
                })?
                .decode()
                .with_context(|| {
                    format!(
                        "Unable to decode image file {:?} for grid menu",
                        image_file_name
                    )
                })?;
            ////let rgba = image.to_rgba8();
            let picture_bar = load_image(image, egui_routine, renderer);
            params.push(GridSelectParams { picture_bar, data });
        }
        Ok(params)
    }
}
