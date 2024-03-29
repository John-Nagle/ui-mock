//! #  piemenu.rs -- N-choice circular menu
//!
//! A general-use 'egui' widget.
//!
//! Draws a circle of pie slices, with text in each box.
//! Each "slice" is clickable.
//
//  ***UNFINISHED***
//
//  Animats
//  March 2024
//
use core::ops::Index;
use egui::{Response, Ui, WidgetText};
use std::f32::consts::PI;
//  Always write TextureId, Vec2, Rect fully qualified to avoid name confusion.

/// PieMenu -- N-choice circular menu
//  The persistent part.
pub struct PieMenu {
    /// Radius of entire button
    radius: f32,
    /// Center radius of button
    center_radius: f32,
    /// Text of button segments, clockwise from top.
    button_text: Vec<egui::RichText>,
    /// Line color
    line_color: egui::Color32,
    /// Background color
    background_color: egui::Color32,
    /// Hover color
    hover_color: egui::Color32,
    /// Hover help text
    hover_text: WidgetText,
}

impl PieMenu {
    /// Image, dimensions of button,
    pub fn new(
        radius: f32,
        center_radius: f32,
        button_text: &[egui::RichText],
        line_color: egui::Color32,
        background_color: egui::Color32,
        hover_color: egui::Color32,
        hover_text: impl Into<WidgetText>,
    ) -> Self {
        Self {
            radius,
            center_radius,
            button_text: button_text.iter().map(|s| s.clone()).collect(),
            line_color,			
            background_color,	
            hover_color,
            hover_text: hover_text.into(),
        }
    }

    //// Decode the click into the user action -- Left, Right, Up, Down, Center, or None.
    //// Users of this widget must call this on Response to find out what the user is asking for.
    /*
    pub fn decode_response(&self, response: &Response) -> NavAction {
        let response = response.interact(egui::Sense::click_and_drag());    // must sense 'dragged' to sense held down.
        if response.dragged() {
            if let Some(interact_pos) = response.hover_pos() {
                //  Compute position relative to center of button.
                //  Do case analysis for left, right, center, up, down.
                let to_vec2 = |p: egui::Pos2| egui::Vec2::new(p.x, p.y); // why not just use one 2d point/vector type?
                let center = (to_vec2(response.rect.min) + to_vec2(response.rect.max)) * 0.5; // Average for center coords
                let rel_pos = to_vec2(interact_pos) - center; // cursor position relative to center of button rect.
                if rel_pos.length() < self.center_button_size {
                    NavAction::Center // inside center button
                } else if rel_pos.x.abs() > rel_pos.y.abs() {
                    // if X dominates
                    if rel_pos.x > 0.0 {
                        NavAction::Right
                    } else {
                        NavAction::Left
                    }
                } else if rel_pos.y < 0.0 {
                    NavAction::Up
                } else {
                    NavAction::Down
                } // < 0 is upwards. Tradition.
            } else {
                NavAction::None //  Must not be in rectangle.
            }
        } else {
            NavAction::None // nothing pushed
        }
    }

    /// Draw the appropriate pressed arrow.
    fn draw_pressed_arrow(&self, ui: &mut Ui, response: Response) -> Response {
        let nav_action = self.decode_response(&response);
        //  Which way to point arrow.
        const ARROW_ROTS: [f32; 6] = [
            0.0,      // None
            PI * 1.5, // Up
            PI * 0.5, // Down
            PI * 1.0, // Left
            PI * 0.0, // Right
            0.0,      // center
        ];
        match nav_action {
            NavAction::None => {}     // no press
            NavAction::Center => {              // center press
                self.center_button.paint_at(ui, response.rect);
            }
            _ => {
                //  Arrow press
                let arrow_rot = ARROW_ROTS[nav_action as usize];
                // Draw the arrow if pressed
                self.arrow.clone()
                .rotate(arrow_rot, egui::Vec2::new(0.5, 0.5))
                .paint_at(ui, response.rect);
            }
        }
        response    // pass through response
    }
    */
}

/// The widget is a circle with clickable pie slices.
impl egui::Widget for &mut PieMenu {
    fn ui(self, ui: &mut Ui) -> Response {
        let response =
            ui.label("Pie menu dummy");          
            ////.frame(true);
        //  Only show hover text when not clicked
        let response = if !response.dragged() {
            response.on_hover_text(self.hover_text.clone())
        } else {
            response
        };
        ////self.draw_pressed_arrow(ui, response) // add arrow indicating pressing
        response
    }
}
