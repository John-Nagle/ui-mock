//! #  statgraph.rs -- statistics graph
//!
//! A general-use 'egui' widget.
//!
//! Graphs of time-dependent variables, for performance measurement.
//
//  Animats
//  April, 2023
//
////use core::ops::Index;
use egui::{Response, Ui, WidgetText};
////use std::f32::consts::PI;
//  Always write TextureId, Vec2, Rect fully qualified to avoid name confusion.

/// StatGraph -- one statistics graph, scrolling time to the left.
//  The persistent part.
pub struct StatGraph {
    hover_text: WidgetText,                // hover text for help       
}

impl StatGraph {
    /// Image, dimensions of button,
    pub fn new(
        hover_text: impl Into<WidgetText>,
    ) -> Self {
        Self {
            hover_text: hover_text.into(),
        }
    }
}

/// The widget is an image button plus a drawn arrow.
impl egui::Widget for &mut StatGraph {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("Statistics graph placeholder")
    }
}
