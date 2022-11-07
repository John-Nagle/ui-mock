//
//  menustart.rs -- Start menu
//
//  Start menu. Shows all the available grids.
//  Fills the whole screen.
//
//  Animats
//  November 2022
//
use libui::{CommonState, MenuGroup, MenuGroupLink};
use libui::{GuiEvent}; //    ***TEMP*** these are moving out of libui
////use super::super::uiinfo::{UiInfo};
use core::cell::RefCell;
use std::rc::Rc;

#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.

/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
//
//  The start screen. A scrolling list of big image buttons, one
//  for each metaverse.
pub struct MenuStart {
    //  No status at this time.
}

impl MenuStart {
    /// Create new, as trait object
    pub fn new_link() -> MenuGroupLink {
        Rc::new(RefCell::new(MenuStart{}))                          // create a trait object to dispatch
    }
}

impl MenuGroup for MenuStart {

    /// Draws the menu set for Start state.
    //  Called on every frame. Do not delay here.
    fn draw(&mut self, state: &mut CommonState) -> bool {                          
        // Insert egui commands here to draw the menus for thie state.
        let ctx = state.platform.context();
        //  Draw the splash screen with a big set of alternative metaverses.
        //
        egui::CentralPanel::default().show(&ctx, |_ui| {
            if let Some(grid) = state.grid_select_window.draw(&ctx) {  // select desired grid
                //  A grid has been selected
                let _ = state.send_gui_event(GuiEvent::LoginTo(grid)); // tell main which grid has been selected.

            }
            state.draw(&ctx); // all the standard windows
        });
        true                // menus must stay visible, not time out
    }
}


