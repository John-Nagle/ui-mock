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
use std::collections::VecDeque;
use crate::t;
use once_cell::sync::OnceCell;

/// All GUI windows persistent state.
#[derive(Default)]
pub struct GuiWindows {
    pub about_window: Option<TextWindow>,            // Help->About
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

/// Text window, with noninteractive content.
//  The persistent part
pub struct TextWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    pub is_open: bool,  // true if open
    message: Vec::<String>, // window text
}

impl TextWindow {
    /// Create persistent text window, multiline
    pub fn new(id: &str, title: &str, message: &[&str]) -> Self {
        TextWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            message: message.iter().map(|s| s.to_string()).collect(),  // array of String is needed
            is_open: true,  // start open
        }
    }
}

impl GuiWindow for TextWindow { 
    /// Draw window of text
    fn draw(&mut self, ctx: &egui::Context) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
            .collapsible(false)
            .open(&mut self.is_open);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            let total_rows = self.message.len();
            egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    if row >= self.message.len() { break }  // prevent scrolling off end
                    ui.label(self.message[row].as_str());
                }
            });
        });
    }
}

/// A scrolling text message window.
//  The persistent part
pub struct MessageWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    lines: VecDeque<String>,         // the text
}

impl MessageWindow {
    /// Create scrollable message window
    pub fn new(id: &str, title: &str, scrollback_limit: usize) -> Self {
        MessageWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            lines: VecDeque::with_capacity(scrollback_limit),
        }        
    }
    
    /// Add a line of text. Consumes string
    pub fn add_line(&mut self, text: String) {
        self.lines.push_back(text);
    }
    
    /// Draw window of text
    pub fn new_window(&self, ctx: &egui::Context) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            ////let total_rows = 10;
            egui::ScrollArea::vertical().show_rows(ui, row_height, self.lines.len(), |ui, row_range| {
                for row in row_range {
                    if row >= self.lines.len() { break }
                    let text = &self.lines[row];
                    ui.label(text);
                }
            });
        });
    }
}
