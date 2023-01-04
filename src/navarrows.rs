//! #  NavArrows.rs -- 4-way navigation button
//!
//! Navigation button with four arrows and a center button.
//! An egui widget.
//
//  Animats
//  Jaunary 2023
//
use egui::{Ui, Response};
use core::ops::Index;
//  Always write TextureId, Vec2, Rect fully qualified to avoid name confusion.

/// NavArrows -- a 4-way arrow with an optional button in the center.
//  The persistent part.
pub struct NavArrows {
    button: (egui::TextureId, egui::Vec2),  // the button image
    arrow: (egui::TextureId, egui::Vec2),   // the arrow image for pressed direction
    center_button_size: f32,                // center button of arrows, if nonzero
}

/// User action - what did the click mean?
#[derive(Debug, PartialEq)]
pub enum NavAction {
        None,
        Up,
        Down,
        Left,
        Right,
        Center
}

/// Enum to integer
impl Index<NavAction> for [usize; 6] {
    type Output = usize;

    fn index(&self, nav_action: NavAction) -> &Self::Output {
        &self[nav_action as usize]
    }
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
        if true {
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
    
    /// Draw the appropriate pressed arrow.
    fn draw_pressed_arrow(&self, ui: &mut Ui, response: Response) -> Response {
        let nav_action = self.decode_response(&response);
        if nav_action == NavAction::None { return response }         // not pressed
        //  Where to draw the arrows, and which way to point them.
        const ARROW_UVS: [egui::Rect;6] = [
            egui::Rect{ min: egui::Pos2 { x: 0.0, y: 0.0 }, max: egui::Pos2 { x: 0.0, y: 0.0 }},    //  None
            egui::Rect{ min: egui::Pos2 { x: 0.0, y: 0.0 }, max: egui::Pos2 { x: 0.0, y: 0.0 }},    //  Up ***
            egui::Rect{ min: egui::Pos2 { x: 0.0, y: 0.0 }, max: egui::Pos2 { x: 0.0, y: 0.0 }},    //  Down ***
            egui::Rect{ min: egui::Pos2 { x: 1.0, y: 1.0 }, max: egui::Pos2 { x: 0.0, y: 0.0 }},    //  Left
            egui::Rect{ min: egui::Pos2 { x: 0.0, y: 0.0 }, max: egui::Pos2 { x: 1.0, y: 1.0 }},    //  Right
            egui::Rect{ min: egui::Pos2 { x: 0.0, y: 0.0 }, max: egui::Pos2 { x: 0.0, y: 0.0 }},    //  Center
        ];
        let arrow_uv = ARROW_UVS[nav_action as usize];
        // Draw the button
        println!("Arrow UV: {:?}", arrow_uv);  // ***TEMP***
        egui::Image::new(self.arrow.0, self.arrow.1).uv(arrow_uv).paint_at(ui, response.rect); // 
        ////egui::Image::new(self.arrow.0, self.arrow.1).paint_at(ui, response.rect); // ***TEMP TEST***
        response
    }
}

impl egui::Widget for &mut NavArrows {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.add(
            egui::widgets::ImageButton::new(
                *&self.button.0,
                *&self.button.1,
                )
                .frame(true)
            );
        self.draw_pressed_arrow(ui, response)
        ////egui::Image::new(self.arrow.0, self.arrow.1).paint_at(ui, result.rect); // ***TEMP TEST***
        ////result     
    }
}

