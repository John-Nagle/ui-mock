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
////use core::ops::Index;
use egui::{Response, Ui, WidgetText};
use std::f32::consts::PI;
//  Always write TextureId, Vec2, Rect fully qualified to avoid name confusion.

const LINE_WIDTH: f32 = 2.0;            // line width for drawing
const TEXT_POS_RADIUS_FRACT: f32 = 0.75;    // how far out to put the text (0..1)

/// PieMenu -- N-choice circular menu
//  The persistent part.
pub struct PieMenu {
    /// Radius of entire button
    radius: f32,
    /// Center radius of button
    center_radius: f32,
    /// Text of button segments, clockwise from top.
    button_text: Vec<egui::WidgetText>,
    /// Line color
    line_color: egui::Color32,
    /// Background color
    background_color: egui::Color32,
    /// Hover color
    hover_color: egui::Color32,
    /// Hover help text
    hover_text: WidgetText,
    /// Cut vectors, one per button text entry
    cut_vectors: Vec<egui::Vec2>,
}

impl PieMenu {
    /// Image, dimensions of button,
    pub fn new(
        radius: f32,
        center_radius: f32,
        button_text: &[egui::WidgetText],
        line_color: egui::Color32,
        background_color: egui::Color32,
        hover_color: egui::Color32,
        hover_text: impl Into<WidgetText>,
    ) -> Self {
        //  Direction vector for each dividing line
        let cut_vector = |n:usize| egui::Vec2::new((2.0*PI/button_text.len() as f32 * (n as f32)).sin(), 
            (2.0*PI/button_text.len() as f32 * (n as f32)).cos());
        Self {
            radius,
            center_radius,
            button_text: button_text.iter().map(|s| (*s).clone()).collect(),
            line_color,			
            background_color,	
            hover_color,
            hover_text: hover_text.into(),
            cut_vectors: (0..button_text.len()).map(|n| cut_vector(n)).collect(),
        }
    }

    
    /// Radius of click menu.
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

/// The widget is a circle with clickable pie slices.
impl egui::Widget for &mut PieMenu {
    fn ui(self, ui: &mut Ui) -> Response {
        let stroke = egui::Stroke::new(LINE_WIDTH, self.line_color);
        let (response, ref mut painter) =
            ui.allocate_painter(egui::Vec2::new(self.radius*2.0, self.radius*2.0), egui::Sense::hover());
        painter.set_clip_rect(response.rect); // clip drawing to widget rect
        let center = response.rect.center();
        //  Outer circle
        painter.circle(center, self.radius, self.background_color, stroke);
        //  Inner circle
        painter.circle(center, self.center_radius, egui::Color32::TRANSPARENT, stroke);
        let pie_cut = |v: egui::Vec2| { painter.line_segment([center + v*self.center_radius, center + v*self.radius], stroke); };
        
        //  Draw the dividing lines and labels
        for (dir, text) in self.cut_vectors.iter().zip(&self.button_text) {
            pie_cut(*dir);   // draw the line
            let font_id = egui::FontId::default();           // for now
            let text_pos = center + (*dir)*(self.center_radius * (1.0 - TEXT_POS_RADIUS_FRACT) + self.radius*TEXT_POS_RADIUS_FRACT);
            painter.text(text_pos, egui::Align2::CENTER_CENTER, text.text().to_string(), font_id, egui::Color32::WHITE);
        }
        response
    }
}
