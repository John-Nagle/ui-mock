//! #  NavArrows.rs -- 4-way navigation button
//!
//! Navigation button with four arrows and a center button.
//! An egui widget.
//
//  Animats
//  Jaunary 2023
//
use egui::{Ui, Response};
//  Always write TextureId or Vec2 fully qualified to avoid name confusion.

/// NavArrows -- a 4-way arrow with an optional button in the center.
//  The persistent part.
pub struct NavArrows {
    button: (egui::TextureId, egui::Vec2),  // the button image
    arrow: (egui::TextureId, egui::Vec2),   // the arrow image for pressed direction
    center_button_size: f32,                // center button of arrows, if nonzero
}

/// User action - what did the click mean?
#[derive(Debug)]
pub enum NavAction {
        None,
        Up,
        Down,
        Left,
        Right,
        Center
}

impl NavArrows {
    /// Image, dimensions of button, 
    pub fn new(button: (egui::TextureId, egui::Vec2),  arrow: (egui::TextureId, egui::Vec2), center_button_size: f32) -> Self {
        Self {
            button,
            arrow,
            center_button_size,
        }
    }

    /// Decode the click into the user action -- Left, Right, Up, Down, Center, or None.
    pub fn decode_response(&self, response: &Response) -> NavAction {
        if response.clicked() {
            if let Some(interact_pos) = response.interact_pointer_pos() {
                //  Compute position relative to center of button.
                let to_vec2 = |p: egui::Pos2| egui::Vec2::new(p.x, p.y);          // why not just use one 2d point/vector type?
                let center = (to_vec2(response.rect.min) + to_vec2(response.rect.max))*0.5;    // Average for center coords
                let rel_pos  = to_vec2(interact_pos) - center;   // cursor position relative to center of button rect.
                if rel_pos.length() < self.center_button_size { 
                    NavAction::Center                   // inside center button
                } else {
                    if rel_pos.x.abs() > rel_pos.y.abs() {  // if X dominates
                        if rel_pos.x > 0.0 { NavAction::Right} else { NavAction::Left }
                    } else {
                        if rel_pos.y < 0.0 { NavAction::Up } else {NavAction::Down }    // < 0 is upwards?
                    }
                }
            } else {
                NavAction::None                     //  Must not be in rectangle.
            }
        } else {
            NavAction::None                         // nothing pushed
        }
    }
}

impl egui::Widget for &mut NavArrows {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            egui::widgets::ImageButton::new(
                *&self.button.0,
                *&self.button.1,
                )
                .frame(true)
            )     
    }
}

