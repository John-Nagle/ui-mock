///#  menunone.rs -- the null MenuGroup
//
//  Just used to get started.
//  Also useful when you need a template for making a MenuGroup.
//  Generic, so that GuiState can instantiate it within libui.
//
//  Animats
//  November 2022
//
use crate::{MenuGroup, MenuGroupLink, CommonState};
use core::cell::RefCell;
use std::rc::Rc;

/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
//
//  The overlay on the main screen. Menus disappear when not used.
//  Cursor to top or bottom of window restores them.
pub struct MenuNone {
}

impl MenuNone  {
    /// Create new, as trait object
    pub fn new_link() -> MenuGroupLink {
        Rc::new(RefCell::new(MenuNone{}))                          // create a trait object to dispatch
    }
}

impl MenuGroup for MenuNone {

    /// Draws the menu set for Login state.
    //  Called on every frame. Do not delay here.
    fn draw(&mut self, _state: &mut CommonState) -> bool {                          
        // Insert egui commands here
        //  Nothing to do, this is MenuNone.
        true
    }
    /// Ident for debug purposes
    fn get_name(&self) -> &'static str {
        "---"
    }
}
