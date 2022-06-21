//
//  gui.rs -- window and menu layout.
//
//  Top menu bar, and a bottom button bar.
//  Both disappear when not used for a while, for
//  a clean game screen.
//
//  Animats
//  June 2022
//
/*
use crate::{UiAssets, UiData};
use super::guimenus;

use egui::{menu, Frame, TextureId};
use rend3::Renderer;
use rend3_egui::EguiRenderRoutine;
use std::sync::Arc;
*/
use crate::t;
use once_cell::sync::OnceCell;

/// All GUI windows persistent state.
#[derive(Default)]
pub struct GuiWindows {
    pub about_window: Option<MessageWindow>,            // Help->About
}

impl GuiWindows {
    /// Draw all live windows
    pub fn draw(&mut self, ctx: &egui::Context) {
        if let Some(w) = &mut self.about_window { w.draw(ctx) }
    }
}

trait GuiWindow {
    fn draw(&mut self, ctx: &egui::Context);
}

/// Message window, with text content
//  The persistent part
pub struct MessageWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    pub is_open: bool,  // true if open
    message: Vec::<String>, // window text
}

impl MessageWindow {
    /// Create persistent text window, multiline
    pub fn new(id: &str, title: &str, message: &[&str]) -> Self {
        MessageWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            message: message.iter().map(|s| s.to_string()).collect(),  // array of String is needed
            is_open: true,  // start open
        }
    }
}

impl GuiWindow for MessageWindow { 
    /// Draw window of text
    //  ***DECIDE HOW TO TURN OFF WHEN DISMISSED*** - delete, or just not open?
    fn draw(&mut self, ctx: &egui::Context) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
            .open(&mut self.is_open);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            let total_rows = 10;
            egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    if row >= self.message.len() { break }  // prevent scrolling off end
                    ui.label(self.message[row].as_str());
                }
            });
        });
    }
}

/// A text window.
//  The persistent part
pub struct TextWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
}

impl TextWindow {
    /// Create persistent text window
    pub fn new(id: &str, title: &str) -> Self {
        TextWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
        }
    }
    
    /// Draw window of text
    pub fn new_window(&self, ctx: &egui::Context) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            let total_rows = 10;
            egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    let text = format!("Row {}/{}", row + 1, total_rows);
                    ui.label(text);
                }
            });
        });
    }
}
