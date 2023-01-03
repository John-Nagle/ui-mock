// # Main program of user interface
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

use libui::{GuiState, GuiParams, GuiAssets, Dictionary, MessageLogger, GuiCommonEvent, SendAnyBoxed};
use libui::{t, get_log_file_name, get_executable_name, panic_dialog};
use std::sync::Arc;
use log::{LevelFilter};
use std::str::FromStr;
use anyhow::{Error};
use libdialog::{UiData, UiInfo, SystemMode, GuiEvent, GridSelectParams, UiAppAssets};
use libdialog::{handle_gui_event};

/// Base level configuration
const MENU_DISPLAY_SECS: u64 = 3;               // hide menus after this much time


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
    #[allow(clippy::new_without_default)]   // don't need a default. Only used once.
    pub fn new() -> Self {
        //  The message channel which allows other things to send to the UI.
        let (event_send_channel, event_recv_channel) = crossbeam_channel::unbounded(); // message channel
        
        AppUi {
            data: None,
            event_recv_channel: Some(event_recv_channel),   // because it will be taken
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
                GuiCommonEvent::ErrorMessage((title, messages)) => {   // display message
                    let msgs: Vec::<&str> = messages.iter().map(|m| m.as_str()).collect();
                    data.gui_state.common_state.add_error_window(title, &msgs);
                }
                GuiCommonEvent::LogMessage(s) => {
                    data.gui_state.common_state.add_msg(s.to_string())
                }
                GuiCommonEvent::Shutdown => {
                    data.gui_state.app_state.change_mode(SystemMode::Shutdown);  // shutdown starts
                    data.quit = true;                   // force quit              
                }                                       // shut down and exit
            }
        } else {
            log::error!("Invalid non GuiEvent/GuiCommonEvent in handle_user_event: {:?}", raw_event);
        }
    }    
    /// Setup of the graphics environment. Returns error.
    fn setup_with_error(
        &mut self,
        window: &winit::window::Window,
        renderer: &Arc<rend3::Renderer>,
        _routines: &Arc<rend3_framework::DefaultRoutines>,
        surface_format: rend3::types::TextureFormat,
    ) -> Result<(), Error> {
        //  Test forcing full screen ***TURNED OFF*** - crashes on Windows
        ////window.set_visible(true); // ***TEMP TEST***
        ////window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        window.set_visible(true);
        window.set_maximized(true);
        ////window.set_decorations(false);
        ////window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        let window_size = window.inner_size();

        // Create the egui render routine
        let mut egui_routine = rend3_egui::EguiRenderRoutine::new(
            renderer,
            surface_format,
            rend3::types::SampleCount::One,
            window_size.width,
            window_size.height,
            window.scale_factor() as f32,
        );

        // Create mesh and calculate smooth normals based on vertices
        let mesh = examplesupport::create_cube_mesh();

        // Add mesh to renderer's world.
        //
        // All handles are refcounted, so we only need to hang onto the handle until we
        // make an object.
        let mesh_handle = renderer.add_mesh(mesh);

        // Add PBR material with all defaults except a single color.
        let material = rend3_routine::pbr::PbrMaterial {
            albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
            transparency: rend3_routine::pbr::Transparency::Blend,
            ..rend3_routine::pbr::PbrMaterial::default()
        };
        let _material_handle = renderer.add_material(material);

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
        let _object_handle = renderer.add_object(object);

        let camera_pitch = std::f32::consts::FRAC_PI_4;
        let camera_yaw = -std::f32::consts::FRAC_PI_4;
        // These values may seem arbitrary, but they center the camera on the cube in
        // the scene
        let camera_location = glam::Vec3A::new(5.0, 7.5, -5.0);
        let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, -camera_pitch, -camera_yaw, 0.0);
        let view = view * glam::Mat4::from_translation((-camera_location).into());

        // Set camera location data
        renderer.set_camera_data(rend3::types::Camera {
            projection: rend3::types::CameraProjection::Perspective {
                vfov: 60.0,
                near: 0.1,
            },
            view,
        });

        // Create a single directional light
        //
        // We need to keep the directional light handle alive.
        let _directional_handle = renderer.add_directional_light(rend3::types::DirectionalLight {
            color: glam::Vec3::ONE,
            intensity: 10.0,
            // Direction will be normalized
            direction: glam::Vec3::new(-1.0, -4.0, 2.0),
            distance: 400.0,
        });

        // Create the winit/egui integration, which manages our egui context for us.
        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: window_size.width as u32,	
                physical_height: window_size.height as u32,
                scale_factor: window.scale_factor(),
                font_definitions: egui::FontDefinitions::default(),
                style: Default::default(),
            });

        //  Icon loading (obsolete)
        let web_icon_bytes = include_bytes!("../../assets/images/iconweb.png");
        let assets = GuiAssets { 
            web_icon: libui::load_canned_icon(web_icon_bytes, &mut egui_routine, renderer),
            };
        //  Icon loading (current). Example content only. Real programs do this at run time, so we can have themes.
        let move_arrows_icon_bytes = include_bytes!("../../assets/images/move-arrows-128.png");
        let rot_arrows_icon_bytes = include_bytes!("../../assets/images/rot-arrows-128.png");
        let ui_app_assets = UiAppAssets {
            move_arrows_icon: libui::load_canned_icon(move_arrows_icon_bytes, &mut egui_routine, renderer),
            rot_arrows_icon: libui::load_canned_icon(rot_arrows_icon_bytes, &mut egui_routine, renderer),
        };    
        let start_time = instant::Instant::now();
        let version = env!("CARGO_PKG_VERSION").to_string();   // Version of main, not libraries
        let asset_dir = std::path::PathBuf::from_str(concat!(env!["CARGO_MANIFEST_DIR"], "/src/assets/"))?; // ***TEST ONLY*** installer dependent
        let mut locale_file = asset_dir.clone();
        locale_file.push("locales");
        locale_file.push("menus.json");
        ////let locale_file = asset_dir.to_string() + "locales/menus.json"; // locale file is under in assets
        let lang = Dictionary::get_translation(&[locale_file])
            .expect("Trouble loading language translation files"); // select language
                                                                   
        //// Detection turned off due to https://github.com/frewsxcv/rust-dark-light/issues/17
        ////let dark_mode = dark_light::detect() == dark_light::Mode::Dark; // True if dark mode
        let dark_mode = false; // ***TEMP*** force dark mode as default
        let log_level = LevelFilter::Warn;                      // warn is default logging level
        println!("Dark mode: {:?} -> {}", dark_light::detect(), dark_mode); // ***TEMP***
        let adapter_info: rend3::ExtendedAdapterInfo = renderer.adapter_info.clone();  // adapter info for About box
        ////println!("Adapter info: {:?}", adapter_info);   // ***TEMP*** 
        const GRID_FILE: &str = "grids.json";
        // Read in the grid select params, which requires reading some images.
        let grid_select_params =
            GridSelectParams:: read_grid_select_params(&std::path::PathBuf::from_str(GRID_FILE)?, &asset_dir, &mut egui_routine, renderer)?;
        //  Initialization data for the GUI.
        //  Just what's needed to bring the GUI up initially
        let params = GuiParams {
            lang,
            version,                        // because we need version of main program, not libs
            asset_dir,
            dark_mode,
            log_level,
            menu_display_secs: MENU_DISPLAY_SECS,
            gpu_info: adapter_info,             // GPU info
            ////grid_select_params,
        };
        let event_send_channel = self.event_send_channel.clone();
        let event_recv_channel = self.event_recv_channel.take().unwrap();
        //  Set initial state of app-level UI info
        let app_state = UiInfo::new(grid_select_params);
        //  Set up main state of the GUI
        let gui_state = GuiState::new(params, assets, platform, event_send_channel, event_recv_channel, app_state);  
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
        self.data.as_mut().unwrap().gui_state.common_state.send_boxed_gui_event(Box::new(GuiEvent::Startup)).unwrap(); // Start up the GUI.
        Ok(())
    }

    
}

/// This is an instance of the Rend3 application framework.
impl rend3_framework::App for AppUi {
    const HANDEDNESS: rend3::types::Handedness = rend3::types::Handedness::Left;

    fn sample_count(&self) -> rend3::types::SampleCount {
        SAMPLE_COUNT
    }
    
    /// Register our loggers.
    //  One logger goes to a file.
    //  One logger goes to a window in the GUI
    fn register_logger(&mut self) {
        let log_file_name = get_log_file_name().expect("Unable to figure out where to put log files.");    // get appropriate name for platform
        let _ = simplelog::CombinedLogger::init(
            vec![
                ////simplelog::TermLogger::new(LevelFilter::Warn, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto),
                simplelog::WriteLogger::new(LevelFilter::Warn, simplelog::Config::default(), std::fs::File::create(*log_file_name.clone()).expect("Unable to create log file")),
                MessageLogger::new_logger(LevelFilter::Warn, self.event_send_channel.clone()),
            ]
        ); 
        log::warn!("Logging to {:?}", log_file_name);   // where the log is going         
    }
    
    /// Setup of the graphics enviornment, popping up a panic dialog on error.
    fn setup(
        &mut self,
        window: &winit::window::Window,
        renderer: &Arc<rend3::Renderer>,
        _routines: &Arc<rend3_framework::DefaultRoutines>,
        surface_format: rend3::types::TextureFormat,
    ) { 
        if let Err(err) = self.setup_with_error(window, renderer, _routines, surface_format) {
            panic_dialog("Start-up failure", &format!("{:?}", err)); // tell user
            panic!("Start up failure: {:?}", err);   // then panic
        }
    }


    /// The event loop. This runs forever, or at least until the user causes an exit.
    fn handle_event(
        &mut self,
        window: &winit::window::Window,
        renderer: &Arc<rend3::Renderer>,
        routines: &Arc<rend3_framework::DefaultRoutines>,
        base_rendergraph: &rend3_routine::base::BaseRenderGraph,
        surface: Option<&Arc<rend3::types::Surface>>,
        resolution: glam::UVec2,
        event: rend3_framework::Event<'_, ()>,
        control_flow: impl FnOnce(winit::event_loop::ControlFlow),
    ) {
        profiling::scope!("Event");


        //  Handle any user events.
        //  Temporarily uses separate queue due to
        //  bug https://github.com/BVE-Reborn/rend3/issues/406
        {   let data = self.data.as_mut().unwrap();
            if !data.gui_state.common_state.event_recv_channel.is_empty() {    // if events queued
                //  Get all events, avoiding double borrow.
                let events: Vec<SendAnyBoxed> = data.gui_state.common_state.event_recv_channel.try_iter().collect(); 
                for ev in events {
                    println!("User event: {:?}", ev);       // ***TEMP***
                    self.handle_user_event(window, ev);
                }
            }
        }
              
        let data = self.data.as_mut().unwrap();
        //  This is where EGUI handles 2D UI events.
        data.gui_state.common_state.platform.handle_event(&event);
        if data.gui_state.common_state.platform.captures_event(&event) {
            ////println!("GUI captured event: {:?}", event);    // ***TEMP TEST***
            return; // 2D UI consumed this event.
        }

        match event {
            rend3_framework::Event::RedrawRequested(..) => {
                profiling::scope!("Redraw.");
                data.gui_state.common_state.platform
                    .update_time(data.start_time.elapsed().as_secs_f64());
                /*
                data.gui_state.platform.begin_frame();

                // Insert egui commands here
                let show_menus = data.gui_state.if_gui_awake();
                let mut inuse = libui::draw(&mut data.gui_state, show_menus); // draws the GUI
                inuse |= is_at_fullscreen_window_top_bottom(window, data); // check if need to escape from full screen
                if inuse {
                    data.gui_state.wake_up_gui();
                }
                let egui::FullOutput {
                    shapes,
                    textures_delta,
                    platform_output,
                    ..
                } = data.gui_state.platform.end_frame(Some(window));
                if !platform_output.events.is_empty() {
                    data.gui_state.wake_up_gui(); // reset GUI idle time.
                    data.gui_state.message_window.add_line(format!(
                        "Platform events: {:?}, {} shapes.",
                        platform_output.events,
                        shapes.len()
                    )); // ***TEMP***
                }

                let paint_jobs = data.gui_state.platform.context().tessellate(shapes);
                */
                let (paint_jobs, textures_delta) = data.gui_state.common_state.draw_all(window);  // build the 2D GUI
                let input = rend3_egui::Input {
                    clipped_meshes: &paint_jobs,
                    textures_delta,
                    context: data.gui_state.common_state.platform.context(),
                };

                profiling::scope!("3D");
                // Get a frame
                let frame = rend3::util::output::OutputFrame::Surface {
                    surface: Arc::clone(surface.unwrap()),
                };

                // Ready up the renderer
                let (cmd_bufs, ready) = renderer.ready();

                // Lock the routines
                let pbr_routine = rend3_framework::lock(&routines.pbr);
                let tonemapping_routine = rend3_framework::lock(&routines.tonemapping);

                // Build a rendergraph
                let mut graph = rend3::graph::RenderGraph::new();

                // Add the default rendergraph without a skybox
                base_rendergraph.add_to_graph(
                    &mut graph,
                    &ready,
                    &pbr_routine,
                    None,
                    &tonemapping_routine,
                    resolution,
                    SAMPLE_COUNT,
                    glam::Vec4::ZERO,
                    glam::Vec4::new(0.10, 0.05, 0.10, 1.0), // Nice scene-referred purple
                );

                // Add egui on top of all the other passes
                let surface = graph.add_surface_texture();
                data.egui_routine.add_to_graph(&mut graph, input, surface);

                // Dispatch a render using the built up rendergraph!
                graph.execute(renderer, frame, cmd_bufs, &ready);
                //  Exit if all done.
                if data.quit {
                    control_flow(winit::event_loop::ControlFlow::Exit);
                } else {
                    control_flow(winit::event_loop::ControlFlow::Poll);
                }
                profiling::finish_frame!(); // end of frame for Tracy purposes
            }
            rend3_framework::Event::MainEventsCleared => {
                window.request_redraw();
            }
            rend3_framework::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    data.egui_routine
                        .resize(size.width, size.height, window.scale_factor() as f32);
                }
                winit::event::WindowEvent::CloseRequested => {
                    control_flow(winit::event_loop::ControlFlow::Exit);
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
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    let _ = input;   // not yet used
                    ////println!("Keyboard event: {:?}", input);    // ***TEMP TEST***
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// The main program.
fn main() {
    #[cfg(feature = "tracy")]
    let _client = tracy_client::Client::start();    // enable profiler if "tracy" feature is on
    #[cfg(feature = "tracy")]
    assert!(tracy_client::Client::is_running());    // if compiled with wrong version of tracy, will fail
    profiling::scope!("Main");
    profiling::register_thread!();
    let app = AppUi::new();
    rend3_framework::start(
        app,
        winit::window::WindowBuilder::new()
            .with_title(get_executable_name().as_str())
            .with_visible(true) 
            .with_maximized(true)   // this is not effective on Linux/X11. Has to be re-done at startup.
    )
}
