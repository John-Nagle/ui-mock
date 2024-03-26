//! # libgui.rs -- The 2D GUI
//
//  Animats
//  June, 2022
//
//  Everything here runs in the main thread.
//  It should not use more than 1% of main thread time.
//  That is checked with Tracy, as being under span "GUI".
//
mod basicintl;
mod guistate;
mod guiutil;
mod menunone;
mod navarrows; // a widget
mod statgraph;  // a widget
mod piemenu; // a widget

use core::any::Any;
use core::cell::RefCell;
use std::rc::Rc;
 
/// Internationalization
pub use basicintl::Dictionary;
pub use guistate::panic_dialog;
/// The main draw function. Called on every frame.
////pub use guimenus::{draw};
pub use guistate::{
    AppState, CommonState, GuiAssets, GuiCommonEvent, GuiParams, ExecutableVersion, GuiState, MessageLogger, SendAny,
    SendAnyBoxed, TextWindow,
};
/// Utility functions.
pub use guiutil::{
    get_asset_dir, get_cache_dir, get_executable_name, get_log_file_name, load_canned_icon,
    load_image, set_default_styles,
};
/// Widgets
pub use navarrows::{NavArrows, NavAction};
pub use piemenu::{PieMenu};
pub use statgraph::{StatGraph};

//  Traits
/// A group of menus. Libui user sets what menus are to be shown.
pub trait MenuGroup {
    /// Draw the item. Called every frame.
    fn draw(&mut self, state: &mut CommonState) -> bool; // returns true if menu is in use
    ///  Pass event to a GUI item. Override to get events.
    fn pass_event(&mut self, _state: &mut CommonState, _event: &SendAnyBoxed) {}
    /// Name for debug and logging purposes only
    fn get_name(&self) -> &'static str; 
    /// For downcasting. Little used, may be removed.
    fn as_any(&self) -> &dyn Any;
    /// For downcasting. 
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A GUI window
pub trait GuiWindow {
    /// Draw the item. Called every frame.
    fn draw(&mut self, ctx: &egui::Context, state: &mut CommonState); // called every frame
    /// Retain this window? Override and set to false for a window to close itself.
    fn retain(&self) -> bool {
        true
    } // override and set to false when done
    ///  Pass event to a GUI item. Override to get events.
    fn pass_event(&mut self, _state: &mut CommonState, _event: &SendAnyBoxed) {}
    /// GetID  of window.
    fn get_id(&self) -> egui::Id; 
    /// For downcasting. 
    fn as_any(&self) -> &dyn Any;
    /// For downcasting. 
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type GuiWindowLink = Rc<RefCell<dyn GuiWindow>>;
pub type MenuGroupLink = Rc<RefCell<dyn MenuGroup>>;
