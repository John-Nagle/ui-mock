//! #  NavArrows.rs -- 4-way navigation button
//!
//! Navigation button with four arrows and a center button.
//! An egui widget.
//
//  Animats
//  Jaunary 2023
//
////use std::rc::Rc;
////use core::cell::RefCell;
////use core::any::Any;
////use libui::{GuiWindow, GuiWindowLink, Dictionary, CommonState};
////use libui::t;
use egui::{Ui, Response};
//  Always write TextureId or Vec2 fully qualified to avoid name confusion.

/// NavArrows -- a 4-way arrow with an optional button in the center.
//  The persistent part.
pub struct NavArrows {
    button_image: egui::TextureId,          // the button image
    button_dims: egui::Vec2,                // dimensions of the button
    center_button_size: f32,                // center button of arrows, if nonzero
 
}

impl NavArrows {
    /// Image, dimensions of button, 
    pub fn new(button_image: egui::TextureId, button_dims: egui::Vec2, center_button_size: f32) -> Self {
        Self {
            button_image,
            button_dims,
            center_button_size,
        }
    }
           
    //  Draw the button
    ////fn draw(&mut self, ctx: &egui::Context, state: &mut CommonState) {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.add(
            egui::widgets::ImageButton::new(
                *&self.button_image,
                *&self.button_dims,
                )
                .frame(true),
            )
            .clicked() {
            // Button clicked upon, do something.
        }
    } 
}

impl egui::Widget for &mut NavArrows {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            egui::widgets::ImageButton::new(
                *&self.button_image,
                *&self.button_dims,
                )
                .frame(true)
            )     
    }
}

