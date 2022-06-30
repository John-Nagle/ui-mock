/// guiutil.rs -- GUI utility functions
//
//  Animats
//  June, 2022


use std::sync::Arc;
use rend3::Renderer;
use image::GenericImageView;
use egui::{Context, TextureId};
use rend3_egui::EguiRenderRoutine;
use egui::FontFamily::{Proportional};
use egui::TextStyle::{Button, Small, Heading, Body, Name, Monospace};
use egui::FontId;


/// Load an icon at compile time. Image is built into executable.
pub fn load_canned_icon(
    image_bytes: &[u8],
    egui_routine: &mut EguiRenderRoutine,
    renderer: &Arc<Renderer>,
) -> TextureId {
    //Images
    println!("Load canned icon: {} bytes", image_bytes.len());  // ***TEMP***
    let image_image = image::load_from_memory(image_bytes).unwrap();
    let image_rgba = image_image.as_rgba8().unwrap().clone().into_raw();
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

/// Set our default styles
//  Called once at startup.
pub fn set_default_styles(ctx: &Context) {
    // Get current context style
    let mut style = (*ctx.style()).clone();
    //  Redefine text_styles
    //  Have to define all of them
    style.text_styles = [
    (Heading, FontId::new(20.0, Proportional)),
    ////(Name("Heading2".into()), FontId::new(25.0, Proportional)),
    ////(Name("Context".into()), FontId::new(23.0, Proportional)),
    (Body, FontId::new(16.0, Proportional)),
    (Monospace, FontId::new(18.0, Proportional)),
    (Button, FontId::new(18.0, Proportional)),
    (Small, FontId::new(14.0, Proportional)),
    ].into();
    // Mutate global style with above changes
    ctx.set_style(style);
}
