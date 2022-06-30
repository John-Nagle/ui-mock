//
//  Main program of user interface
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
use libui;
use libui::{GuiState, GuiParams, GuiEvent, GuiAssets, GridSelectParams, Dictionary};
use std::sync::Arc;
use log::{LevelFilter};

/// Configuration
const MENU_DISPLAY_SECS: u64 = 3; // hide menus after this much time

pub struct UiData {
    //  These keep reference-counted Rend3 objects alive.
    _object_handle: rend3::types::ObjectHandle,
    _material_handle: rend3::types::MaterialHandle,
    _directional_handle: rend3::types::DirectionalLightHandle,

    egui_routine: rend3_egui::EguiRenderRoutine,
    start_time: instant::Instant,
    quit: bool,                                 // global quit flag

    //  The 2D GUI
    gui_state: GuiState,                        // state of the GUI
}

impl UiData {

}

const SAMPLE_COUNT: rend3::types::SampleCount = rend3::types::SampleCount::One;

/// The application.
#[derive(Default)]
pub struct Ui {
    data: Option<UiData>,
}

impl Ui {
    /// Handle user-created event.
    //  This is how the GUI and other parts of the
    //  system communicate with the main event loop.
    pub fn handle_user_event(&mut self, event: GuiEvent) {
        let data = self.data.as_mut().unwrap();
        match event {
            GuiEvent::OpenReplay(_path_buf) => {     // open a replay file
                data.gui_state.unimplemented_msg(); // ***TEMP***
            }
            GuiEvent::LoginTo(grid) => {
                println!("Now log into this: {}", grid.name);
            }
            GuiEvent::SaveReplay(_path_buf) => {     // save a replay file
                data.gui_state.unimplemented_msg(); // ***TEMP***
            }
            GuiEvent::ErrorMessage((title, messages)) => {   // display message
                let msgs: Vec::<&str> = messages.iter().map(|m| m.as_str()).collect();
                data.gui_state.add_error_window(&title, &msgs);
            }
            GuiEvent::Quit => {
                data.quit = true;                   // force quit              
            }                                       // shut down and exit
        }
    }
}

/// True if cursor is at the top or bottom of the screen in full screen mode.
//  This is how you get the menus back from a totally clean window.
pub fn is_at_fullscreen_window_top_bottom(window: &winit::window::Window, data: &UiData) -> bool {
    const NEAR_EDGE: f32 = 5.0; // if within this many pixels of top or bottom
                                ////if !window.fullscreen().is_some() { return false; }               // only meaningful for full screen
    let inner_size = window.inner_size(); // sizes of window
    let ctx = data.gui_state.platform.context();
    if let Some(pos) = ctx.pointer_interact_pos() {
        // check for pointer at top or bottom of window
        ////println!("pos: {:?}, height: {}", pos, inner_size.height);
        pos.y < NEAR_EDGE || pos.y + NEAR_EDGE > (inner_size.height as f32)
    } else {
        false
    }
}

/// This is an instance of the Rend3 application framework.
impl rend3_framework::App for Ui {
    const HANDEDNESS: rend3::types::Handedness = rend3::types::Handedness::Left;

    fn sample_count(&self) -> rend3::types::SampleCount {
        SAMPLE_COUNT
    }

    /// Setup of the graphics environment
    fn setup(
        &mut self,
        window: &winit::window::Window,
        renderer: &Arc<rend3::Renderer>,
        _routines: &Arc<rend3_framework::DefaultRoutines>,
        surface_format: rend3::types::TextureFormat,
    ) {
        //  Test forcing full screen ***TURNED OFF*** - crashes on Windows
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
        let mesh = create_mesh();

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

        //  Icon loading
        let web_icon_bytes = include_bytes!("../../images/iconweb.png");
        let assets = GuiAssets { 
            web_icon: libui::load_canned_icon(web_icon_bytes, &mut egui_routine, renderer),
            //  ***TEMP*** until this stuff gets loaded at run time.
            replay_bar: libui::load_canned_icon(include_bytes!("../../images/replaybar.png"), &mut egui_routine, renderer),    
            placeholder_a_bar: libui::load_canned_icon(include_bytes!("../../images/placeholdera.png"), &mut egui_routine, renderer), 
            placeholder_b_bar: libui::load_canned_icon(include_bytes!("../../images/placeholderb.png"), &mut egui_routine, renderer), 
            };

        let start_time = instant::Instant::now();
        let version = env!("CARGO_PKG_VERSION").to_string();   // Version of main, not libraries
        let locale_file = concat!(env!["CARGO_MANIFEST_DIR"], "/src/locales/menus.json"); // ***TEST ONLY*** Installer-dependent
        let lang = Dictionary::get_translation(&[locale_file])
            .expect("Trouble loading language translation files"); // select language
                                                                   
        //// Detection turned off due to https://github.com/frewsxcv/rust-dark-light/issues/17
        ////let dark_mode = dark_light::detect() == dark_light::Mode::Dark; // True if dark mode
        let dark_mode = true; // ***TEMP*** force dark mode as default
        let log_level = LevelFilter::Warn;                      // warn is default logging level
        println!("Dark mode: {:?} -> {}", dark_light::detect(), dark_mode); // ***TEMP***
        let adapter_info: rend3::ExtendedAdapterInfo = renderer.adapter_info.clone();  // adapter info for About box
        ////println!("Adapter info: {:?}", adapter_info);   // ***TEMP*** 
        let grid_select_params = get_grid_select_params(&assets);                                                                
        //  Initialization data for the GUI.
        //  Just what's needed to bring the GUI up initially
        let params = GuiParams {
            lang,
            version,                        // because we need version of main program, not libs
            dark_mode,
            log_level,
            menu_display_secs: MENU_DISPLAY_SECS,
            gpu_info: adapter_info,             // GPU info
            grid_select_params,
        };
        let gui_state = GuiState::new(params, assets, platform);     // all the fixed and popup windows
        self.data = Some(UiData {
            _object_handle,
            _material_handle,
            _directional_handle,
            egui_routine,
            start_time,
            gui_state,
            quit: false,
        });
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
            if !data.gui_state.event_recv_channel.is_empty() {    // if events queued
                //  Get all events, avoiding double borrow.
                let events: Vec<GuiEvent> = data.gui_state.event_recv_channel.try_iter().collect(); 
                for ev in events {
                    println!("User event: {:?}", ev);       // ***TEMP***
                    self.handle_user_event(ev);
                }
            }
        }
              
        let data = self.data.as_mut().unwrap();
        //  This is where EGUI handles 2D UI events.
        data.gui_state.platform.handle_event(&event);
        if data.gui_state.platform.captures_event(&event) {
            return; // 2D UI consumed this event.
        }

        match event {
            rend3_framework::Event::RedrawRequested(..) => {
                profiling::scope!("Redraw.");
                data.gui_state.platform
                    .update_time(data.start_time.elapsed().as_secs_f64());
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

                let input = rend3_egui::Input {
                    clipped_meshes: &paint_jobs,
                    textures_delta,
                    context: data.gui_state.platform.context(),
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
                        data.gui_state.wake_up_gui();
                    } // make menus reappear on focus
                }
                winit::event::WindowEvent::CursorEntered { .. } => {
                    data.gui_state.wake_up_gui(); // either entering or leaving makes menus reappear
                }
                winit::event::WindowEvent::CursorLeft { .. } => {
                    data.gui_state.wake_up_gui();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    #[cfg(feature = "tracy")]
    let _client = tracy_client::Client::start();    // enable profiler if "tracy" feature is on
    #[cfg(feature = "tracy")]
    assert!(tracy_client::Client::is_running());    // if compiled with wrong version of tracy, will fail
    profiling::scope!("Main");
    profiling::register_thread!();
    let app = Ui::default();
    rend3_framework::start(
        app,
        winit::window::WindowBuilder::new()
            .with_title("UI mockup")
            .with_maximized(true),
    )
}

fn vertex(pos: [f32; 3]) -> glam::Vec3 {
    glam::Vec3::from(pos)	
}

fn create_mesh() -> rend3::types::Mesh {
    let vertex_positions = [
        // far side (0.0, 0.0, 1.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        // near side (0.0, 0.0, -1.0)
        vertex([-1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // right side (1.0, 0.0, 0.0)
        vertex([1.0, -1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        // left side (-1.0, 0.0, 0.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // top (0.0, 1.0, 0.0)
        vertex([1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        // bottom (0.0, -1.0, 0.0)
        vertex([1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
    ];

    let index_data: &[u32] = &[
        0, 1, 2, 2, 3, 0, // far
        4, 5, 6, 6, 7, 4, // near
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // top
        20, 21, 22, 22, 23, 20, // bottom
    ];

    rend3::types::MeshBuilder::new(vertex_positions.to_vec(), rend3::types::Handedness::Left)
        .with_indices(index_data.to_vec())
        .build()
        .unwrap()
}

/// Dummy of get grid select params.
//  These will come from a file in future,
//  with an entry for each supported metaverse.
fn get_grid_select_params(assets: &GuiAssets) -> Vec<GridSelectParams> {
    let replay_file_dummy_grid = GridSelectParams {
        name: "Replay file".to_string(),
        picture_bar: assets.replay_bar,
        web_url: "https://www.animats.com/viewer".to_string(),
        login_url: None,
    };
    let placeholder_a = GridSelectParams {
        name: "Second Life".to_string(),
        picture_bar: assets.placeholder_a_bar,
        web_url: "https://www.secondlife.com/viewer".to_string(),
        login_url: Some("https://login.aditi.lindenlab.com/cgi-bin/login.cgi".to_string()),
    };
    let placeholder_b = GridSelectParams {
        name: "OsGrid".to_string(),
        picture_bar: assets.placeholder_b_bar,
        web_url: "https://www.osgrid.org".to_string(),
        login_url: Some("http://login.osgrid.org".to_string()),
    };
    vec![replay_file_dummy_grid, placeholder_a, placeholder_b]
}
