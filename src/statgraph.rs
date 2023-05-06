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
////use egui::plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;
//  Always write TextureId, Vec2, Rect fully qualified to avoid name confusion.

/// Time series of equally spaced data
pub struct TimeSeries {
    /// Length
    length: usize,
    /// The points
    values: VecDeque<f32>,
}

impl TimeSeries {
    /// Usual new
    pub fn new(length: usize) -> Self {
        assert!(length > 0);
        Self {
            length,
            values: VecDeque::with_capacity(length)
        }
    }
    
    /// Add to time series
    pub fn push(&mut self, v: f32) {
        while self.values.len() >= self.length { // if oversize, drain
            let _ = self.values.pop_front();
        }
        self.values.push_back(v);
    }
    
    /// Set new length. Discards data if needed
    pub fn set_length(&mut self, length: usize) {
        assert!(length > 0);
        self.length = length;
        while self.values.len() >= length { // if oversize, drain
            let _ = self.values.pop_front();
        }
    }
    
    /// Return time series as a generator of plot points
    pub fn as_plot_points(&self) -> impl Iterator<Item = egui::plot::PlotPoint> +'_ {
        self.values.iter().enumerate().map(|(i, &y)| egui::plot::PlotPoint::new(i as f64, y as f64))
    }
}
    

/// StatGraph -- one statistics graph, scrolling time to the left.
//  The persistent part.
pub struct StatGraph {
    /// Title of graph
    title: WidgetText,
    /// Y range
    y_range: [f32;2],
    /// Unique ID
    id: egui::Id,
    /// The actual data.
    time_series: TimeSeries,

}

impl StatGraph {
    /// Usual new
    pub fn new(
        title: impl Into<WidgetText>,
        y_range: [f32;2],
        length: usize,
        id: &str,
    ) -> Self {
        Self {
            title: title.into(),
            y_range,
            id: egui::Id::new(id),
            time_series: TimeSeries::new(length),
        }
    }
    
    /// Add a value to the time series.
    pub fn push(&mut self, v: f32) {
        self.time_series.push(v);
    }
}

/// The widget is a graph
impl egui::Widget for &mut StatGraph {
    /// Draw. Called every frame if open.
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label(self.title.clone());
            let values = self.time_series.as_plot_points();  // returns an iterator.
            //  Unfortunately, Line wont't yet take an iterator.
            let temp_values_1: Vec<egui::plot::PlotPoint> = values.collect();   // so we have to make a list of values
            let temp_values = egui::plot::PlotPoints::Owned(temp_values_1);
            egui::plot::Plot::new(self.id)
                .view_aspect(5.0)
                ////.width(250.0)
                .include_x(0.0)
                .include_x(self.time_series.length as f64)
                .include_y(self.y_range[0])
                .include_y(self.y_range[1])
                .show_x(false)
                .show(ui, |plot_ui| plot_ui.line(egui::plot::Line::new(temp_values).fill(0.0))).response
        }).response
    }
}
