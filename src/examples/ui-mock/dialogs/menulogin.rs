//
//  menulogin.rs -- Start menu
//
//  Fills the whole screen and puts up a dialog.
//
//  Animats
//  November 2022
//
use libui::{FixedStateInfo, GuiEvent, MenuGroup, MenuGroupLink};
use libui::t;
use core::cell::RefCell;
use std::rc::Rc;
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.

/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
//
//  The start screen. A scrolling list of big image buttons, one
//  for each metaverse.
pub struct MenuLogin {
}

impl MenuLogin {
    /// Create new, as trait object
    pub fn new_link() -> MenuGroupLink {
        Rc::new(RefCell::new(MenuLogin{}))                          // create a trait object to dispatch
    }
}

impl MenuGroup for MenuLogin {

    /// Draws the menu set for Login state.
    //  Called on every frame. Do not delay here.
    fn draw(&mut self, state: &mut FixedStateInfo) -> bool {                          
        // Login to a grid
        let ctx = state.platform.context();

        //  Top menu bar
        egui::TopBottomPanel::top("grid_login_container").show(&ctx, |ui| {
            if ui.button(t!("menu.unimplemented", state.get_lang())).clicked() {
                ////state.selected_grid = None;                 // clear grid selection ***MOVE TO APP LEVEL***
                let _ = state.send_gui_event(GuiEvent::Startup);       // Back to startup state
                ////state.change_mode(SystemMode::Start);       // back to start state
            }
        });

        //  Central panel
        egui::CentralPanel::default().show(&ctx, |ui| {
            //  Login dialog
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |_ui| {
            })
        });
        state.draw(&ctx); // all the standard windows
        true
    }
}


