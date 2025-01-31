//! # Main program of user interface
//
//  This is a mockup of the user interface for
//  a metaverse viewer. It has windows and menus,
//  but doesn't do anything except display them.
//
//  Should be portable to Linux, Windows, and Apple desktop platforms.
//
//  Modeled after the "egui" example from "Rend3".
//
//  Animats
//  June 2022
//
mod examplesupport;
mod libdialog;

use anyhow::Error;
use libdialog::handle_gui_event;
use libdialog::{
    GridSelectParams, GuiEvent, StatisticsEvent, SystemMode, UiAppAssets, UiData, UiInfo,
};
use libui::{get_executable_name, get_log_file_name, panic_dialog, t};
use libui::{
    Dictionary, ExecutableVersion, GuiAssets, GuiCommonEvent, GuiParams, GuiState, MessageLogger,
    SendAnyBoxed,
};
use log::LevelFilter;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::fmt::Debug;

//  Debug use only.
use winit::event_loop::ActiveEventLoop;
use winit::event::StartCause;

/// Hide menus after this much time
const MENU_DISPLAY_SECS: u64 = 3; 
/// Statistics this often
const STATISTICS_INTERVAL: Duration = Duration::new(1, 0); 
/// Turn on for verbose event printing.
const PRINT_ALL_EVENTS: bool = true;
/// Formatting for timestamps in debug print
const TIME_FORMAT_DESCRIPTION: &[time::format_description::FormatItem] =
    time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:3]");
/// Anti-aliasing
const SAMPLE_COUNT: rend3::types::SampleCount = rend3::types::SampleCount::One;

/// The application.
pub struct AppUi {
    data: Option<UiData>,
    //  UI event channel.
    //  We have to do this at the outer level so the logger can access it early.
    event_send_channel: crossbeam_channel::Sender<SendAnyBoxed>,
    event_recv_channel: Option<crossbeam_channel::Receiver<SendAnyBoxed>>,
}

impl AppUi {
    /// Create the top-level user interface struct.
    //  This owns everything.
    #[allow(clippy::new_without_default)] // don't need a default. Only used once.
    pub fn new() -> Self {
        //  The message channel which allows other things to send to the UI.
        let (event_send_channel, event_recv_channel) = crossbeam_channel::unbounded(); // message channel

        AppUi {
            data: None,
            event_recv_channel: Some(event_recv_channel), // because it will be taken
            event_send_channel: event_send_channel.clone(),
        }
    }

    /// Handle user-created event.
    //  This is how the GUI and other parts of the
    //  system communicate with the main event loop.
    pub fn handle_user_event(&mut self, window: &winit::window::Window, raw_event: SendAnyBoxed) {
        //  Events can be a GuiEvent or a GuiCommonEvent.
        //  The dynamic typing is to get the definition of GuiEvent out of
        //  libui.
        let data = self.data.as_mut().unwrap();
        if let Some(event) = raw_event.downcast_ref::<GuiEvent>() {
            log::warn!("GuiEvent: {:?}", event);
            handle_gui_event(data, window, event); // main GUI event handler switch
        } else if let Some(event) = raw_event.downcast_ref::<GuiCommonEvent>() {
            //  Handle standard utility-type events.
            match event {
                GuiCommonEvent::ErrorMessage((title, messages)) => {
                    // display message
                    let msgs: Vec<&str> = messages.iter().map(|m| m.as_str()).collect();
                    data.gui_state.common_state.add_error_window(title, &msgs);
                }
                GuiCommonEvent::LogMessage(s) => data.gui_state.common_state.add_msg(s.to_string()),
                GuiCommonEvent::ErrorShutdown(s) => {
                    log::error!("Error shutdown: {}",s); // in real programs do a popup.
                    data.gui_state.app_state.change_mode(SystemMode::Shutdown); // shutdown starts
                    data.quit = true; // force quit
                }

                GuiCommonEvent::Shutdown => {
                    data.gui_state.app_state.change_mode(SystemMode::Shutdown); // shutdown starts
                    data.quit = true; // force quit
                } // shut down and exit
            }
        } else {
            log::error!(
                "Invalid non GuiEvent/GuiCommonEvent in handle_user_event: {:?}",
                raw_event
            );
        }
    }
    /// Setup of the graphics environment. Returns error.
    fn setup_with_error(
        &mut self,
        context: rend3_framework::SetupContext<'_>,
    ) -> Result<(), Error> {
        //  Test forcing full screen ***TURNED OFF*** - crashes on Windows
        ////window.set_visible(true); // ***TEMP TEST***
        ////window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        ////window.set_visible(true);
        ////window.set_maximized(true);
        ////window.set_decorations(false);
        ////context.windowing.as_ref().unwrap().window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        ////context.windowing.as_ref().unwrap().window.set_maximized(true);
        let window_size = context.resolution;

        // Create the egui render routine
        let mut egui_routine = rend3_egui::EguiRenderRoutine::new(
            context.renderer,
            context.surface_format,
            rend3::types::SampleCount::One,
            window_size.x,
            window_size.y,
            context.scale_factor,
        );

        // Create mesh and calculate smooth normals based on vertices
        let mesh = examplesupport::create_cube_mesh();

        // Add mesh to renderer's world.
        //
        // All handles are refcounted, so we only need to hang onto the handle until we
        // make an object.
        let mesh_handle = match context.renderer.add_mesh(mesh) {
            Ok(handle) => handle,
            Err(e) => return Err(anyhow::anyhow!("{:?}", e)), // because WGPU errors are not Send, cloneable, or copyable.
        };

        // Add PBR material with all defaults except a single color.
        let material = rend3_routine::pbr::PbrMaterial {
            albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
            transparency: rend3_routine::pbr::Transparency::Blend,
            ..rend3_routine::pbr::PbrMaterial::default()
        };
        let _material_handle = context.renderer.add_material(material);

        // Combine the mesh and the material with a location to give an object.
        let object = rend3::types::Object {
            mesh_kind: rend3::types::ObjectMeshKind::Static(mesh_handle),
            material: _material_handle.clone(),
            transform: glam::Mat4::IDENTITY,
        };

        // Creating an object will hold onto both the mesh and the material
        // even if they are deleted.
        //
        // We need to keep the object handle alive.
        let _object_handle = context.renderer.add_object(object);

        let camera_pitch = std::f32::consts::FRAC_PI_4;
        let camera_yaw = -std::f32::consts::FRAC_PI_4;
        // These values may seem arbitrary, but they center the camera on the cube in
        // the scene
        let camera_location = glam::Vec3A::new(5.0, 7.5, -5.0);
        let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, -camera_pitch, -camera_yaw, 0.0);
        let view = view * glam::Mat4::from_translation((-camera_location).into());

        // Set camera location data
        context.renderer.set_camera_data(rend3::types::Camera {
            projection: rend3::types::CameraProjection::Perspective {
                vfov: 60.0,
                near: 0.1,
            },
            view,
        });

        // Create a single directional light
        //
        // We need to keep the directional light handle alive.
        let _directional_handle =
            context
                .renderer
                .add_directional_light(rend3::types::DirectionalLight {
                    color: glam::Vec3::ONE,
                    intensity: 10.0,
                    // Direction will be normalized
                    direction: glam::Vec3::new(-1.0, -4.0, 2.0),
                    distance: 400.0,
                    resolution: 2048, // ***NOT SURE ABOUT THIS***
                });
        // Create the egui context
        let egui_context = egui::Context::default();
        //  Copied from Rend3 egui example.
        let platform = if let Some(windowing) = context.windowing {
            Some(egui_winit::State::new(
                egui_context.clone(),
                egui::ViewportId::default(),
                &windowing.window,
                Some(context.scale_factor),
                None,
                None,
            ))
        } else {
            None
        };
        //  Icon loading (obsolete)
        let web_icon_bytes = include_bytes!("../../assets/images/iconweb.png");
        let assets = GuiAssets {
            web_icon: libui::load_canned_icon(web_icon_bytes, &mut egui_routine, context.renderer),
        };
        //  Icon loading (current). Example content only. Real programs do this at run time, so we can have themes.
        let move_arrows_icon_bytes = include_bytes!("../../assets/images/move-arrows-128.png");
        let rot_arrows_icon_bytes = include_bytes!("../../assets/images/rot-arrows-128.png");
        let pressed_arrow_icon_bytes =
            include_bytes!("../../assets/images/arrow-pressed-right-64.png");
        let pressed_button_icon_bytes = include_bytes!("../../assets/images/center-pressed-64.png");
        let ui_app_assets = UiAppAssets {
            move_arrows_icon: libui::load_canned_icon(
                move_arrows_icon_bytes,
                &mut egui_routine,
                context.renderer,
            ),
            rot_arrows_icon: libui::load_canned_icon(
                rot_arrows_icon_bytes,
                &mut egui_routine,
                context.renderer,
            ),
            pressed_arrow_icon: libui::load_canned_icon(
                pressed_arrow_icon_bytes,
                &mut egui_routine,
                context.renderer,
            ),
            pressed_button_icon: libui::load_canned_icon(
                pressed_button_icon_bytes,
                &mut egui_routine,
                context.renderer,
            ),
        };
        let start_time = Instant::now();
        let _version = env!("CARGO_PKG_VERSION").to_string(); // Version of main, not libraries
        let asset_dir =
            std::path::PathBuf::from_str(concat!(env!["CARGO_MANIFEST_DIR"], "/src/assets/"))?; // ***TEST ONLY*** installer dependent
        let mut locale_file = asset_dir.clone();
        locale_file.push("locales");
        locale_file.push("menus.json");
        ////let locale_file = asset_dir.to_string() + "locales/menus.json"; // locale file is under in assets
        let lang = Dictionary::get_translation(&[locale_file])
            .expect("Trouble loading language translation files"); // select language

        //// Detection turned off due to https://github.com/frewsxcv/rust-dark-light/issues/17
        ////let dark_mode = dark_light::detect() == dark_light::Mode::Dark; // True if dark mode
        let dark_mode = false; // ***TEMP*** force dark mode as default
        let log_level = LevelFilter::Warn; // warn is default logging level
        println!("Dark mode: {:?} -> {}", dark_light::detect(), dark_mode); // ***TEMP***
        let adapter_info: rend3::ExtendedAdapterInfo = context.renderer.adapter_info.clone(); // adapter info for About box
                                                                                              ////println!("Adapter info: {:?}", adapter_info);   // ***TEMP***
        const GRID_FILE: &str = "grids.json";
        // Read in the grid select params, which requires reading some images.
        let grid_select_params = GridSelectParams::read_grid_select_params(
            &std::path::PathBuf::from_str(GRID_FILE)?,
            &asset_dir,
            &mut egui_routine,
            context.renderer,
        )?;

        //  Info about the executable program.
        const BUILD_ID: &str = git_version::git_version!(); // build info, computed at compile time
        let executable_version = ExecutableVersion {
            program_name: env!("CARGO_PKG_NAME").trim().to_string(),
            major_version: env!("CARGO_PKG_VERSION_MAJOR").trim().to_string(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").trim().to_string(),
            patch_version: env!("CARGO_PKG_VERSION_PATCH").trim().to_string(),
            git_build_id: BUILD_ID.trim().to_string(),
        };
        //  Initialization data for the GUI.
        //  Just what's needed to bring the GUI up initially
        let params = GuiParams {
            lang,
            executable_version, // because we need version of main program, not libs
            asset_dir,
            dark_mode,
            log_level,
            menu_display_secs: MENU_DISPLAY_SECS,
            gpu_info: adapter_info, // GPU info
                                    ////grid_select_params,
        };
        let event_send_channel = self.event_send_channel.clone();
        let event_recv_channel = self.event_recv_channel.take().unwrap();
        //  Set initial state of app-level UI info
        let app_state = UiInfo::new(grid_select_params);
        //  Set up main state of the GUI
        let gui_state = GuiState::new(
            params,
            assets,
            platform.expect("No platform"),
            egui_context,
            event_send_channel,
            event_recv_channel,
            app_state,
        );
        self.data = Some(UiData {
            _object_handle,
            _material_handle,
            _directional_handle,
            egui_routine,
            start_time,
            gui_state,
            quit: false,
            ui_app_assets,
        });
        self.data
            .as_mut()
            .unwrap()
            .gui_state
            .common_state
            .send_boxed_gui_event(Box::new(GuiEvent::Startup))
            .unwrap(); // Start up the GUI.
        Ok(())
    }

    /// Frame statistics update, called once per frame.
    /// Only does something once per second.
    fn frame_statistics_update(data: &mut UiData) {
        //  Calculate frame statistics
        let now = Instant::now();
        if data.gui_state.app_state.frame_statistics.frame_update(now) < STATISTICS_INTERVAL {
            return;
        } // too soon?
          //  Only once per second past this point.
        let (frame_count, statistics_time, worst_frame_time) =
            data.gui_state.app_state.frame_statistics.reset();
        //  ***MORE***
        //  Send 1 second statistics to GUI
        let stats_msg = StatisticsEvent {
            frame_time_average: statistics_time.as_secs_f32() / (frame_count.max(1) as f32),
            frame_time_longest: worst_frame_time.as_secs_f32(),
            ..Default::default()
        };
        //  Pass directly to GUI, without going through a queue.
        data.gui_state.common_state.pass_event(Box::new(stats_msg));
    }
}

/// This is an instance of the Rend3 application framework.
impl rend3_framework::App for AppUi {

    /// Called for every event.
    /// Debug use only.
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if PRINT_ALL_EVENTS {
            let now = time::OffsetDateTime:: now_utc();
            println!("{} Event: {:?}", now.format(TIME_FORMAT_DESCRIPTION).unwrap(), cause);
        }
    }
   
    /// Which handedness do we want.
    //  Has to have a function due to mod to Rend3 API due to mod to Winit. 
    fn get_handedness(&self) -> rend3::types::Handedness {
        rend3::types::Handedness::Left
    }

    fn sample_count(&self) -> rend3::types::SampleCount {
        SAMPLE_COUNT
    }

    /// Register our loggers.
    //  One logger goes to a file.
    //  One logger goes to a window in the GUI
    fn register_logger(&mut self) {
        let log_file_name =
            get_log_file_name().expect("Unable to figure out where to put log files."); // get appropriate name for platform
        let _ = simplelog::CombinedLogger::init(vec![
            ////simplelog::TermLogger::new(LevelFilter::Warn, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto),
            simplelog::WriteLogger::new(
                LevelFilter::Warn,
                simplelog::Config::default(),
                std::fs::File::create(*log_file_name.clone()).expect("Unable to create log file"),
            ),
            MessageLogger::new_logger(LevelFilter::Error, self.event_send_channel.clone()),
        ]);
        log::warn!("Logging to {:?}", log_file_name); // where the log is going
    }

    /// Setup of the graphics enviornment, popping up a panic dialog on error.
    fn setup(&mut self, context: rend3_framework::SetupContext<'_>) {
        if let Err(err) = self.setup_with_error(context) {
            panic_dialog("Start-up failure", &format!("{:?}", err)); // tell user
            panic!("Start up failure: {:?}", err); // then panic
        }
    }

    /// The redraw event, which has its own trait item
    fn handle_redraw(&mut self, context: rend3_framework::RedrawContext<'_>) {
        profiling::scope!("Redraw.");
        if PRINT_ALL_EVENTS {
            println!("{} Redraw.", time::OffsetDateTime:: now_utc().format(TIME_FORMAT_DESCRIPTION).unwrap());
        }
        //  Calculate frame statistics
        let data = self.data.as_mut().unwrap();
        Self::frame_statistics_update(data);
        //  Build the 2D GUI
        let (paint_jobs, textures_delta) = data
            .gui_state
            .common_state
            .draw_all(context.window.as_ref().unwrap()); // build the 2D GUI
        let input = rend3_egui::Input {
            clipped_meshes: &paint_jobs,
            textures_delta,
            context: data.gui_state.common_state.context.clone(),
        };

        profiling::scope!("3D");
        // Swap the instruction buffers so that our frame's changes can be processed.
        context.renderer.swap_instruction_buffers();
        // Evaluate our frame's world-change instructions
        let mut eval_output = context.renderer.evaluate_instructions();
        // Lock the routines
        let pbr_routine = rend3_framework::lock(&context.routines.pbr);
        let tonemapping_routine = rend3_framework::lock(&context.routines.tonemapping);

        // Build a rendergraph
        let mut graph = rend3::graph::RenderGraph::new();

        // Import the surface texture into the render graph.
        let frame_handle = graph.add_imported_render_target(
            context.surface_texture,
            0..1,
            0..1,
            rend3::graph::ViewportRect::from_size(context.resolution),
        );

        // Add the default rendergraph without a skybox
        context.base_rendergraph.add_to_graph(
            &mut graph,
            rend3_routine::base::BaseRenderGraphInputs {
                eval_output: &eval_output,
                routines: rend3_routine::base::BaseRenderGraphRoutines {
                    pbr: &pbr_routine,
                    skybox: None,
                    tonemapping: &tonemapping_routine,
                },
                target: rend3_routine::base::OutputRenderTarget {
                    handle: frame_handle,
                    resolution: context.resolution,
                    samples: SAMPLE_COUNT,
                },
            },
            rend3_routine::base::BaseRenderGraphSettings {
                ambient_color: glam::Vec4::ZERO,
                clear_color: glam::Vec4::new(0.10, 0.05, 0.10, 1.0), // Nice scene-referred purple
            },
        );

        // Add egui on top of all the other passes
        data.egui_routine
            .add_to_graph(&mut graph, input, frame_handle);

        // Dispatch a render using the built up rendergraph!
        graph.execute(context.renderer, &mut eval_output);

        //  Exit if all done.
        profiling::finish_frame!(); // end of frame for Tracy purposes
        if data.quit {
            context.event_loop_window_target.as_ref().unwrap().exit();
            ////control_flow(winit::event_loop::ControlFlow::Exit);
        }
    }
    
    /// We do window redraw only when everything else goes idle.
    /// Otherwise, arrow key processing is too slow.
    /// Ref: Rend3 issue 570.
    fn handle_redraw_done(&mut self, _window: &winit::window::Window) {}

    /// The event loop. This runs forever, or at least until the user causes an exit.
    fn handle_event(
        &mut self,
        context: rend3_framework::EventContext<'_>,
        event: winit::event::Event<()>,
    ) {
        profiling::scope!("Event");

        //  Handle any user events.
        //  Temporarily uses separate queue due to
        //  bug https://github.com/BVE-Reborn/rend3/issues/406
        {
            let data = self.data.as_mut().unwrap();
            if !data.gui_state.common_state.event_recv_channel.is_empty() {
                // if events queued
                //  Get all events, avoiding double borrow.
                let events: Vec<SendAnyBoxed> = data
                    .gui_state
                    .common_state
                    .event_recv_channel
                    .try_iter()
                    .collect();
                for ev in events {
                    println!("User event: {:?}", ev); // ***TEMP***
                    self.handle_user_event(context.window.as_ref().unwrap(), ev);
                }
            }
        }

        let data = self.data.as_mut().unwrap();

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                //  This is where EGUI handles 2D UI events.
                ////if data.gui_state.common_state.platform.on_window_event(&data.gui_state.common_state.context, &event).consumed {
                if data
                    .gui_state
                    .common_state
                    .platform
                    .on_window_event(&context.window.as_ref().unwrap(), &event)
                    .consumed
                {
                    return; // 2D UI consumed this event.
                }

                match event {
                    winit::event::WindowEvent::Resized(size) => {
                        data.egui_routine.resize(
                            size.width,
                            size.height,
                            context.window.as_ref().unwrap().scale_factor() as f32,
                        );
                    }
                    winit::event::WindowEvent::Focused(gained) => {
                        if gained {
                            data.gui_state.common_state.wake_up_gui();
                        } // make menus reappear on focus
                    }
                    winit::event::WindowEvent::CursorEntered { .. } => {
                        data.gui_state.common_state.wake_up_gui(); // either entering or leaving makes menus reappear
                    }
                    winit::event::WindowEvent::CursorLeft { .. } => {
                        data.gui_state.common_state.wake_up_gui();
                    }
                    winit::event::WindowEvent::KeyboardInput { .. } => {
                        ////let _ = input; // not yet used
                        ////println!("Keyboard event: {:?}", input);    // ***TEMP TEST***
                    }                
                    
                    _ => {}
                }
            }
            //  About to wait, so time to redraw
            winit::event::Event::AboutToWait => {
                if PRINT_ALL_EVENTS {
                    println!("{} AboutToWait.", time::OffsetDateTime:: now_utc().format(TIME_FORMAT_DESCRIPTION).unwrap());
                }
                context.window.as_ref().unwrap().request_redraw();
            }            
            _ => {}
        }
    }
}

/// The main program.
fn main() {
    #[cfg(feature = "tracy")]
    let _client = tracy_client::Client::start(); // enable profiler if "tracy" feature is on
    #[cfg(feature = "tracy")]
    assert!(tracy_client::Client::is_running()); // if compiled with wrong version of tracy, will fail
    profiling::scope!("Main");
    profiling::register_thread!();
    let app = AppUi::new();
    rend3_framework::start(
        app,
        winit::window::WindowAttributes::default()
            .with_title(get_executable_name().as_str())
            .with_visible(true)
            .with_maximized(true), // this is not effective on Linux/X11. Has to be re-done at startup.
    )
}
