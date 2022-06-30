//
//  guiwindows.rs -- window and menu layout.
//
//  Top menu bar, and a bottom button bar.
//  Both disappear when not used for a while, for
//  a clean game screen.
//
//  Animats
//  June 2022
//
use std::collections::VecDeque;
use std::path::PathBuf;
use anyhow::{anyhow, Error};
use simplelog::LevelFilter;
use super::basicintl::Dictionary;
use super::guiutil;
use crate::t;
use rend3::{ExtendedAdapterInfo};
/// Configuration
const MESSAGE_SCROLLBACK_LIMIT: usize = 200;   // max scrollback for message window

/// User events sent to the main event loop
#[derive(Debug, Clone)]
pub enum GuiEvent {
    OpenReplay(PathBuf),                            // open a replay file
    SaveReplay(PathBuf),                            // save into a replay file
    LoginTo(GridSelectParams),                      // ask for login params
    ////Login(ConnectInfo),                         // login dialog result
    ErrorMessage((String, Vec<String>)),            // pops up an warning dialog (title, [text])
    Quit                                            // shut down and exit
}

/// Initial values needed to initialize the GUI.
pub struct GuiParams {
    pub version: String,                            // main program version
    pub lang: Dictionary,                           // translation dictionary for chosen language
    pub dark_mode: bool,                            // true if in dark mode
    pub log_level: LevelFilter,                     // logging level
    pub menu_display_secs: u64,                     // (secs) display menus for this long
    pub gpu_info: ExtendedAdapterInfo,              // GPU info
    pub grid_select_params: Vec<GridSelectParams>,  // grid info
}

/// Assets used in displaying the GUI.
#[derive(Default)]
pub struct GuiAssets {
    pub web_icon: egui::TextureId,
    pub replay_bar: egui::TextureId,
    //  ***TEMP*** will be loaded from file at startup
    pub placeholder_a_bar: egui::TextureId,
    pub placeholder_b_bar: egui::TextureId,
}

/// GUI states.
//  The main states of the system.
//  This is a state machine
#[derive(Debug, Copy, Clone)]
pub enum SystemMode {
    Start,  // idle, waiting for grid selection
            // -> Login, Replay. Exit
    Login,  // login di	alog is up.
            // -> Connected, Start
    Connecting, // Connecting to server
            // -> Connected, Start
    Connected, // Fully connected, all menus live
            // -> Shutdown
    Replay, // in replay mode, some menus live
            // -> Shutdown
    Shutdown, // shutting down and cleaning up
            // -> Exit, Start
    Exit, // Program exits
}


/// All GUI windows persistent state.
pub struct GuiState {
    //  Data needed in GUI
    pub params: GuiParams,                      // starting params
    //  Assets - images, etc.
    pub assets: GuiAssets,
    //  Platform data for context
    pub platform: egui_winit_platform::Platform,
    //  Primary system mode
    system_mode: SystemMode,                // primary operating mode
    //  Fixed, reopenable windows.
    pub grid_select_window: GridSelectWindow,   // used at start
    pub about_window: Option<TextWindow>,       // Help->About
    pub message_window: MessageWindow,          // miscellaneous messages ***TEMP***
    //  Disposable dynamic windows
    temporary_windows: Vec<Box<dyn GuiWindow>>,
    //  Misc.
    msg_ok: String,                             // translated OK message
    unique_id: usize,                           // unique ID, serial
    last_interaction_time: instant::Instant,    // time of last user 2D interaction
    pub event_send_channel: crossbeam_channel::Sender<GuiEvent>,
    pub event_recv_channel: crossbeam_channel::Receiver<GuiEvent>,
}

impl GuiState {

    /// Usual new
    pub fn new(params: GuiParams, assets: GuiAssets, platform: egui_winit_platform::Platform) -> GuiState {
        //  Set up base windows.
        let message_window = MessageWindow::new("Messages", t!("window.messages", &params.lang), MESSAGE_SCROLLBACK_LIMIT);
        let grid_select_window = GridSelectWindow::new("Grid select", t!("window.grid_select", &params.lang), &assets, params.grid_select_params.clone());
        //  Set up defaults
        guiutil::set_default_styles(&platform.context());  // set up color and text defaults.
        //  Some common words need translations handy
        let msg_ok =  t!("menu.ok", &params.lang).to_string();
        let (event_send_channel, event_recv_channel) = crossbeam_channel::unbounded(); // message channel
        GuiState {
            platform,
            message_window,
            grid_select_window,
            params,
            assets,
            about_window: None,
            temporary_windows: Vec::new(),
            msg_ok,
            unique_id: 0,
            last_interaction_time: instant::Instant::now(),
            system_mode: SystemMode::Start,
            event_send_channel,
            event_recv_channel,    
        }
    }

    /// Draw all live windows
    pub fn draw(&mut self, ctx: &egui::Context) {
        //  Semi-permanent windows
        if let Some(w) = &mut self.about_window { w.draw(ctx) }
        //  Temporary windows
        for w in &mut self.temporary_windows { w.draw(ctx) }  // draw all temporaries
        self.temporary_windows.retain(|w| w.retain());  // keep only live ones
    }
    
    /// General window add
    pub fn add_window(&mut self, window: Box<dyn GuiWindow>) -> Result<(), Error> {
        //  Check for duplicate window
        for w in &self.temporary_windows {
            if w.get_id() == window.get_id() {
                return Err(anyhow!("Duplicate id for window"));
            }
        }
        self.temporary_windows.push(window);
        Ok(())
    }
    
    /// Get a unique ID, starting from 1.
    pub fn get_unique_id(&mut self) -> egui::Id {
        self.unique_id += 1;                    // serial number increment
        egui::Id::new(self.unique_id)           // unique egui Id
    }
    
    /// Get translations for current language
    pub fn get_lang(&self) -> &Dictionary {
        &self.params.lang
    }
    
    /// Add error message popup.
    //  Handle common errors       
    pub fn add_error_window(&mut self, title: &str, message: &[&str]) {
        //  Create a window with text and an "OK" button.
        let w = TextWindow::new(self.get_unique_id(), title, message, Some(self.msg_ok.as_str()));
        self.add_window(Box::new(w)).expect("Duplicate error msg ID");  // had better not be a duplicate
    }
    
    /// Pop up "unimplemented" message.
    pub fn unimplemented_msg(&mut self) {
        self.add_error_window(t!("menu.unimplemented", self.get_lang()), &[t!("menu.unimplemented", self.get_lang())]);
    }
    
    /// Call this for anything that indicates the GUI should be awakened to show menus.
    pub fn wake_up_gui(&mut self) {
        self.last_interaction_time = instant::Instant::now();
    }
    
    /// Should GUI be shown?
    pub fn if_gui_awake(&self) -> bool {
        self.last_interaction_time.elapsed().as_secs() < self.params.menu_display_secs
    }
    
    pub fn change_mode(&mut self, new_mode: SystemMode) {
        log::info!("System state change: {:?} -> {:?}", self.system_mode, new_mode);
        self.system_mode = new_mode;                    // do state change
    }
    
    pub fn get_mode(&self) -> SystemMode {
        self.system_mode
    }
    
    /// Sends a user event to the event loop.
    //  This ought to use winit events, but we can't do that yet
    //  because of bug https://github.com/BVE-Reborn/rend3/issues/406
    pub fn send_gui_event(&self, event: GuiEvent) -> Result<(), Error> {
        Ok(self.event_send_channel.send(event)?)    // send
    }
}

pub trait GuiWindow {
    fn draw(&mut self, ctx: &egui::Context);    // called every frame
    fn retain(&self) -> bool { true }           // override and set to false when done
    fn get_id(&self) -> egui::Id;               // get ID of window
}

/// Text window, with noninteractive content.
//  The persistent part
pub struct TextWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    is_open: bool,  // true if open
    message: Vec::<String>, // window text
    dismiss_button: Option::<String>, // for "OK" button if desired
}

impl TextWindow {
    /// Create persistent text window, multiline
    pub fn new(id: egui::Id, title: &str, message: &[&str], dismiss_button: Option::<&str>) -> Self {
        TextWindow {
            id,
            title: title.to_string(),
            message: message.iter().map(|s| s.to_string()).collect(),  // array of String is needed
            is_open: true,  // start open
            dismiss_button: dismiss_button.map(|s| s.to_string())   // to string if present, else none

        }
    }
    
    /// Reopen previously closed window, with old contents.
    pub fn reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for TextWindow { 
    /// Draw window of text
    fn draw(&mut self, ctx: &egui::Context) {
        if self.is_open {
            let mut dismissed = false;          // true if dismiss button pushed
            let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
                .collapsible(false);
            //  Only add window close button in title bar if no "OK" button.
            let window = if self.dismiss_button.is_none() { window.open(&mut self.is_open) } else { window };
            window.show(ctx, |ui| {
                //  Scroll area
                //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
                let total_rows = self.message.len();
                if total_rows == 1 {
                    //  Single-line message, center it.
                    ui.vertical_centered(|ui| {
                        ui.label(self.message[0].as_str());
                    });
                } else {
                    //  Multi-line message, can become scrollable.
                    egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                        for row in row_range {
                            if row >= self.message.len() { break }  // prevent scrolling off end
                            ui.label(self.message[row].as_str());
                        }
                    });
                };
                //  Dismiss button, if present
                if let Some(s) = &self.dismiss_button {
                    ui.vertical_centered(|ui| {
                        if ui.add(egui::Button::new(s)).clicked() {
                            dismissed = true;                       // dismiss
                        }
                    });
                };
            });
            if dismissed { self.is_open = false; } // do here to avoid borrow clash
        }
    }
    /// If this is in the dynamic widgets list, drop if retain is false.
    fn retain(&self) -> bool {
        self.is_open
    }
    
    //  Access ID
    fn get_id(&self) -> egui::Id {
        self.id
    }   
}

/// A scrolling text message window.
//  The persistent part
pub struct MessageWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    lines: VecDeque<String>,         // the text
}

impl MessageWindow {
    /// Create scrollable message window
    pub fn new(id: &str, title: &str, scrollback_limit: usize) -> Self {
        MessageWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            lines: VecDeque::with_capacity(scrollback_limit),
        }        
    }
    
    /// Add a line of text. Consumes string
    pub fn add_line(&mut self, text: String) {
        self.lines.push_back(text);
    }
    
    /// Draw window of text
    pub fn draw(&self, ctx: &egui::Context) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            ////let total_rows = 10;
            egui::ScrollArea::vertical().show_rows(ui, row_height, self.lines.len(), |ui, row_range| {
                for row in row_range {
                    if row >= self.lines.len() { break }
                    let text = &self.lines[row];
                    ui.label(text);
                }
            });
        });
    }
}

/// Basic info about a grid for the splash page
#[derive(Debug, Clone)]
pub struct GridSelectParams {
    pub name: String,
    pub picture_bar: egui::TextureId,
    pub web_url: String,
    pub login_url: Option<String>,              // if none, this is a replay  
}

/// The grid selection window.
//  Appears at startup.
//  The persistent part
pub struct GridSelectWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    web_icon: egui::TextureId,  // icon for web button
    grids: Vec<GridSelectParams>, // available grids
}

impl GridSelectWindow {
    /// Create scrollable message window
    pub fn new(id: &str, title: &str, assets: &GuiAssets, grids: Vec<GridSelectParams>) -> Self {
        GridSelectWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            web_icon: assets.web_icon,
            grids
        }        
    }
        
    /// Draw window of text
    pub fn draw(&self, ctx: &egui::Context)-> Option<GridSelectParams> {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id);
        let mut result = None;  // what, if anything, was clicked upon
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            ////let row_height = ui.text_style_height(&text_style);
            let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            //  Add image and website link to each row
            egui::ScrollArea::vertical().show_rows(ui, row_height, self.grids.len(), |ui, row_range| {
                for row in row_range {
                    let grid = &self.grids[row];
                    ui.label(&grid.name);
                    ui.horizontal(|ui| {
                        //  Grid select
                        if ui.add(
                            egui::widgets::ImageButton::new(
                                grid.picture_bar,
                                egui::Vec2::new(1024.0,128.0),
                                )
                            .frame(true),
                        )
                        .clicked()
                        {
                            result = Some(grid.clone());
                        }
                        //  Grid page open
                        if ui.add(
                            egui::widgets::ImageButton::new(
                                self.web_icon,
                                egui::Vec2::new(128.0,128.0),
                                )
                            .frame(true),
                        )
                        .clicked()
                        {   //  Clicking on web icon opens web page for that grid
                            match webbrowser::open(&grid.web_url) {
                                Ok(_) => {},
                                Err(e) => {
                                    //  Popup if trouble
                                    /* ***MORE*** need access to state
                                    let errmsg = format!("{:?}",e);
                                    let messages = [t!("message.web_error", state.get_lang()), errmsg.as_str()];
                                    state.add_error_window(t!("window.internet_error", state.get_lang()), &messages);
                                    */
                                }
                            }
                        }
                    });
                }
            });
        });
        result      // selected grid, or None
    }
}