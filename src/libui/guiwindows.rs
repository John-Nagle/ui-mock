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
use std::collections::VecDeque;

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
    fn draw(&mut self, ctx: &egui::Context);    // called every frame
    fn retain(&self) -> bool { true }           // override and set to false when done
}

/// Text window, with noninteractive content.
//  The persistent part
pub struct TextWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    is_open: bool,  // true if open
    message: Vec::<String>, // window text
    dismiss_button: Option::<String>, // for "OK" button if desired
}

impl TextWindow {
    /// Create persistent text window, multiline
    pub fn new(id: &str, title: &str, message: &[&str], dismiss_button: Option::<&str>) -> Self {
        TextWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            message: message.iter().map(|s| s.to_string()).collect(),  // array of String is needed
            is_open: true,  // start open
            ////dismiss_button: match dismiss_button { Some(s) => Some(s.to_string()), _ => None },
            dismiss_button: dismiss_button.map(|s| s.to_string())   // to string if present, else none

        }
    }
    
    /// Reopen previously closed window, with old contents.
    pub fn reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for TextWindow { 
    /// Draw window of text
    fn draw(&mut self, ctx: &egui::Context) {
        if self.is_open {
            let mut dismissed = false;          // true if dismiss button pushed
            let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
                .collapsible(false);
            //  Only add window close button in title bar if no "OK" button.
            let window = if self.dismiss_button.is_none() { window.open(&mut self.is_open) } else { window };
            window.show(ctx, |ui| {
                //  Scroll area
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
                //  Dismiss button, if present
                if let Some(s) = &self.dismiss_button {
                    ui.vertical_centered(|ui| {
                        if ui.add(egui::Button::new(s)).clicked() {
                            dismissed = true;                       // dismiss
                        }
                    });
                };
            });
            if dismissed { self.is_open = false; } // do here to avoid borrow clash
        }
    }
    /// If this is in the dynamic widgets list, drop if retain is false.
    fn retain(&self) -> bool {
        self.is_open
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
