//
//  gui.rs -- window and menu layout.
//
//  Top menu bar, and a bottom button bar.
//  Both disappear when not used for a while, for
//  a clean game screen.
//
//  Animats
//  June 2022
//
use crate::{UiAssets, UiData};
use egui::{menu, Frame, TextureId};
use rend3::Renderer;
use rend3_egui::EguiRenderRoutine;
use std::sync::Arc;

/// Configuration
const HELP_PAGE: &str =
    "https://github.com/John-Nagle/ui-mock#ui-mock---mockup-of-a-game-type-user-interface";

/// GUI utility functions

/// Load an icon at compile time. Image is built into executable.
pub fn load_canned_icon(
    image_bytes: &[u8],
    egui_routine: &mut EguiRenderRoutine,
    renderer: &Arc<Renderer>,
) -> TextureId {
    //Images
    let image_image = image::load_from_memory(image_bytes).unwrap();
    let image_rgba = image_image.as_rgba8().unwrap().clone().into_raw();
    use image::GenericImageView;
    let dimensions = image_image.dimensions();
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    //  Create and return texture
    rend3_egui::EguiRenderRoutine::create_egui_texture(
        &mut egui_routine.internal,
        renderer,
        format,
        &image_rgba,
        dimensions,
        Some("Canned icon"),
    )
}

/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn update_gui(assets: &UiAssets, data: &mut UiData, show_menus: bool) -> bool {
    ////let data = example.data.as_mut().unwrap();
    // Insert egui commands here
    let ctx = data.platform.context();
    //  Top menu bar
    if show_menus {
        egui::TopBottomPanel::top("menu_bar").show(&ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // File menu
                    {
                        if ui.button("Open").clicked() {
                            // Open menu entry
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Viewer session file to play back")
                                .add_filter("json", &["json"])
                                .pick_file()
                            {
                                let picked_path = Some(path.display().to_string());
                                println!("File picked: {}", picked_path.unwrap());
                            }
                        }
                    }
                    {
                        if ui.button("Quit").clicked() {
                            //  Quit menu entry
                            data.quit = true;
                        }
                    }
                });
                ui.menu_button("Help", |ui| {
                    // Help menu
                    if ui.button("Help").clicked() {
                        // Help menu entry
                        webbrowser::open(HELP_PAGE).expect("failed to open URL");
                        // ***MAKE THIS NON FATAL***
                    }
                });
            });
        });

        //  Bottom button panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(&ctx, |ui| {
                ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT; // transparent button background
                if ui
                    .add(
                        egui::widgets::ImageButton::new(
                            (*assets).rust_logo,
                            egui::Vec2::splat(64.0),
                        )
                        .frame(true),
                    )
                    .clicked()
                {
                    println!("Clicked on Rust button");
                }
                ////ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT; // transparent button background
            });
    }
    ctx.is_pointer_over_area() // True if GUI is in use
}
