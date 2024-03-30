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

/// Line width for drawing
const LINE_WIDTH: f32 = 2.0;
/// How far out to put the text (0..1)
const TEXT_POS_RADIUS_FRACT: f32 = 0.7; 
/// Density of selected wedges
const SELECTED_GAMMA: f32 = 0.75;
/// Density of unselected wedges
const UNSELECTED_GAMMA: f32 = 0.25;

/// PieMenu -- N-choice circular menu
//  The persistent part.
pub struct PieMenu {
    /// Radius of entire button
    radius: f32,
    /// Center radius of button
    center_radius: f32,
    /// Text of button segments, clockwise from top.
    button_text: Vec<egui::WidgetText>,
    /// Text color
    text_color: egui::Color32,
    /// Line color
    line_color: egui::Color32,
    /// Background color
    background_color: egui::Color32,
    /// Cut vectors, one per button text entry
    cut_vectors: Vec<egui::Vec2>,
}

impl PieMenu {
    /// Image, dimensions of button,
    pub fn new(
        radius: f32,
        center_radius: f32,
        button_text: &[egui::WidgetText],
        text_color: egui::Color32,
        line_color: egui::Color32,
        background_color: egui::Color32,
    ) -> Self {
        //  Direction vector for each dividing line
        let cut_vector = |n:usize| egui::Vec2::new((2.0*PI/button_text.len() as f32 * (n as f32)).sin(), 
            (2.0*PI/button_text.len() as f32 * (n as f32)).cos());
        Self {
            radius,
            center_radius,
            button_text: button_text.iter().map(|s| (*s).clone()).collect(),
            text_color,
            line_color,			
            background_color,	
            cut_vectors: (0..button_text.len()).map(|n| cut_vector(n)).collect(),
        }
    }

    
    /// Radius of click menu.
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl PieMenu {
    /// Draw pie-shaped wedge with hole in center.
    fn draw_wedge(&self, painter: &mut egui::Painter, center: egui::Pos2, wedge_number: usize, fill_color: egui::Color32) {
        assert!(wedge_number < self.button_text.len());     // must be in range
        let dir1 = self.cut_vectors[wedge_number];   // first vector of wedge
        let dir2 = self.cut_vectors[(wedge_number + 1) % self.button_text.len()]; // second vector of wedge
        let interp = |f: f32| (dir1 * (1.0 - f) + dir2 * f).normalized();
        //  Approximate a wedge with curved inner and outer edges. 
        //  A Bezier curve would be more elegant, but this is close enough.
        let points = vec![
            center + dir1 * self.center_radius,
            center + dir1 * (self.radius - LINE_WIDTH*0.5),
            center + interp(0.25) * (self.radius - LINE_WIDTH*0.5),
            center + interp(0.375) * (self.radius - LINE_WIDTH*0.5),
            center + interp(0.50) * (self.radius - LINE_WIDTH*0.5),
            center + interp(0.625) * (self.radius - LINE_WIDTH*0.5),
            center + interp(0.75) * (self.radius - LINE_WIDTH*0.5),
            center + dir2 * (self.radius - LINE_WIDTH*0.5),
            center + dir2 * self.center_radius,
            center + interp(0.75) * self.center_radius,            
            center + interp(0.5) * self.center_radius,
            center + interp(0.25) * self.center_radius,            
        ];
        let stroke = egui::Stroke::new(LINE_WIDTH, self.line_color);
        let wedge  = epaint::PathShape::convex_polygon(points, fill_color, stroke);
        painter.add(wedge);
    }
    
    fn draw_pie_cut(&self, painter: &mut egui::Painter, center: egui::Pos2, wedge_number: usize, stroke: egui::Stroke) {
        let v = self.cut_vectors[wedge_number];
        painter.line_segment([center + v*self.center_radius, center + v*self.radius], stroke); 
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
        let hovered_wedge = Some(2);  // ***TEMP***    
        //  Draw wedges and text first.
        let text_pos_on_radial = |dir: egui::Vec2| dir*(self.center_radius * (1.0 - TEXT_POS_RADIUS_FRACT) + self.radius*TEXT_POS_RADIUS_FRACT);
        for n in 0..self.button_text.len() {
            //  Do we want to emphasize this wedge?
            let gamma = if let Some(selected_wedge_number) = hovered_wedge {
                if n == selected_wedge_number {
                    SELECTED_GAMMA
                } else {
                    UNSELECTED_GAMMA
                }
            } else {
                UNSELECTED_GAMMA
            };
            self.draw_wedge(painter, center, n, self.background_color.gamma_multiply(gamma));    // background color for wedge          
            let m = (n+1) % self.button_text.len();
            let text_pos = center + (text_pos_on_radial(self.cut_vectors[n]) + text_pos_on_radial(self.cut_vectors[m])) * 0.5;
            let font_id = egui::FontId::default();           // for now
            let text = &self.button_text[n];
            painter.text(text_pos, egui::Align2::CENTER_CENTER, text.text().to_string(), font_id, self.text_color);            
        }
        //  Finally draw all the opaque lines on top.
        //  Draw outer circle.
        painter.circle_stroke(center, self.radius - LINE_WIDTH*0.5, stroke);
        //  Draw inner circle.
        //  This drawing clear thing doesn't work.
        painter.circle_stroke(center, self.center_radius, stroke);  
        for n in 0..self.button_text.len() {
            self.draw_pie_cut(painter, center, n,  stroke);
        }

        response
    }
}
