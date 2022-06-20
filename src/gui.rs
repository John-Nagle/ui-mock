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
use super::guimenus;
use egui::{menu, Frame, TextureId};
use rend3::Renderer;
use rend3_egui::EguiRenderRoutine;
use std::sync::Arc;
////#[macro_use]
use crate::t;
use once_cell::sync::OnceCell;

/// Configuration
const HELP_PAGE: &str =
    "https://github.com/John-Nagle/ui-mock#ui-mock---mockup-of-a-game-type-user-interface";

/// Grey background for button area.
//  This really should be a gradient.
const TRANSLUCENT_GREY_ALPHA: u8 = 48;
const TRANSLUCENT_GREY: u8 = 32;
const TRANSLUCENT_GREY_COLOR: u8 =
    ((TRANSLUCENT_GREY_ALPHA as u16 * TRANSLUCENT_GREY as u16) / 256) as u8;
const TRANSLUCENT_GREY_COLOR32: egui::Color32 = egui::Color32::from_rgba_premultiplied(
    TRANSLUCENT_GREY_COLOR,
    TRANSLUCENT_GREY_COLOR,
    TRANSLUCENT_GREY_COLOR,
    TRANSLUCENT_GREY_ALPHA,
);

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
    profiling::scope!("Gui");
                           
    // Insert egui commands here
    let ctx = data.platform.context();
    if data.dark_mode {
        ctx.set_visuals(egui::Visuals::dark()); // dark mode if needed
    } else {
        ctx.set_visuals(egui::Visuals::light()); // Switch to light mode
    }
    //  Top menu bar

    if show_menus {
        egui::TopBottomPanel::top("menu_bar").show(&ctx, |ui| {
            ui.menu_button(t!("menu.avatar", &data.lang), |ui| {                                       
                    // Avatar menu
                    if ui.button(t!("menu.preferences", &data.lang)).clicked() {
                        // Preferences menu entry
                        guimenus::manu_preferences(ui, data);
                    }

                    if ui.button(t!("menu.quit", &data.lang)).clicked() {
                        guimenus::menu_quit(ui, data);
                    }
                });
            menu::bar(ui, |ui| {
                ui.menu_button(t!("menu.developer", &data.lang), |ui| {                                       
                    // Replay file menu
                    if ui.button(t!("menu.loadreplay", &data.lang)).clicked() {
                        // Open menu entry
                        guimenus::menu_open_replay(ui, data);
                     } 

                });
                ui.menu_button(t!("menu.help", &data.lang), |ui| {
                    // Help menu
                    if ui.button(t!("menu.help", &data.lang)).clicked() {
                        // Help menu entry
                        webbrowser::open(HELP_PAGE).expect("failed to open URL");
                        // ***MAKE THIS NON FATAL***
                    }
                });
            });
        });

        //  Bottom button panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(Frame::none().fill(TRANSLUCENT_GREY_COLOR32))
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
    //  Non-menu items
    data.message_window.new_window(&ctx);
    //  Finish
    ctx.is_pointer_over_area() // True if GUI is in use
}

/// A text window.
//  The persistent part
pub struct TextWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
}

impl TextWindow {
    /// Create persistent text window
    pub fn new(id: &str, title: &str) -> Self {
        TextWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
        }
    }

    /// Draw window of text
    pub fn new_window(&self, ctx: &egui::Context) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            let total_rows = 10;
            egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    let text = format!("Row {}/{}", row + 1, total_rows);
                    ui.label(text);
                }
            });
        });
    }
}
