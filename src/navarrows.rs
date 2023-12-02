//! #  NavArrows.rs -- 4-way navigation button
//!
//! A general-use 'egui' widget.
//! A navigation button with four directional arrows and a optional center button.
//! The user must provide a background button image, and an image of an arrow.
//! The arrow image should point to the right, and will be rotated into the 
//! Up, Down, and Left positions.
//
//  Animats
//  Jaunary 2023
//
use core::ops::Index;
use egui::{Response, Ui, WidgetText};
use std::f32::consts::PI;
//  Always write TextureId, Vec2, Rect fully qualified to avoid name confusion.

/// NavArrows -- a 4-way arrow with an optional button in the center.
//  The persistent part.
pub struct NavArrows<'a> {
    button: (egui::TextureId, egui::Vec2), // the button image
    arrow: egui::Image<'a>,                // the arrow image for pressed direction
    center_button: egui::Image<'a>,        // the center button
    hover_text: WidgetText,                // hover text for help       
    center_button_size: f32,               // center button of arrows, if nonzero
}

/// User action - what did the click mean?
#[derive(Debug, PartialEq)]
pub enum NavAction {
    None,
    Up,
    Down,
    Left,
    Right,
    Center,
}

/// Enum to integer
impl Index<NavAction> for [usize; 6] {
    type Output = usize;

    fn index(&self, nav_action: NavAction) -> &Self::Output {
        &self[nav_action as usize]
    }
}

impl NavArrows<'_> {
    /// Image, dimensions of button,
    pub fn new(
        button: (egui::TextureId, egui::Vec2),
        arrow: (egui::TextureId, egui::Vec2),
        center_button: (egui::TextureId, egui::Vec2),
        center_button_size: f32,
        hover_text: impl Into<WidgetText>,
    ) -> Self {
        Self {
            button,
            arrow: egui::Image::new((arrow.0, arrow.1)), // preprocess a bit
            center_button: egui::Image::new((center_button.0, center_button.1)), // preprocess a bit
            center_button_size,
            hover_text: hover_text.into(),
        }
    }

    /// Decode the click into the user action -- Left, Right, Up, Down, Center, or None.
    /// Users of this widget must call this on Response to find out what the user is asking for.
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
}

/// The widget is an image button plus a drawn arrow.
impl egui::Widget for &mut NavArrows<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response =
            ui.add(egui::widgets::ImageButton::new((self.button.0, self.button.1))            
            .frame(false));
        //  Only show hover text when not clicked
        let response = if !response.dragged() {
            response.on_hover_text(self.hover_text.clone())
        } else {
            response
        };
        self.draw_pressed_arrow(ui, response) // add arrow indicating pressing
    }
}
