///  libgui.rs -- The 2D GUI
//
//  Animats
//  June, 2022
//
//  Everything here runs in the main thread.
//  It should not use more than 1% of main thread time.
//  That is checked with Tracy, as being under span "GUI".
//
mod basicintl;
mod guiutil;
mod guistate;
mod menunone;   // the empty menu
//  Imports for here
use std::rc::Rc;
use core::cell::RefCell;
use core::any::Any;

/// The main draw function. Called on every frame.
////pub use guimenus::{draw};
pub use guistate::{GuiParams, GuiAssets, GuiState, AppState, CommonState, GuiCommonEvent, MessageLogger, SendAny, SendAnyBoxed, TextWindow};
pub use guistate::{panic_dialog};
/// Utility functions.
pub use guiutil::{load_canned_icon, load_image, set_default_styles, get_log_file_name, get_executable_name, get_asset_dir, get_cache_dir};
/// Internationalization
pub use basicintl::{Dictionary};
/// Dialogs
////pub use dialogs::guilogin::{LoginParams, LoginDialogWindow, LoginDestination};
////pub use dialogs::guigrid::{GridSelectParams};

//  Traits
/// A group of menus. Libui user sets what menus are to be shown.
pub trait MenuGroup {
    fn draw(&mut self, state: &mut CommonState) -> bool;    // returns true if menu is in use
    fn get_name(&self) -> &'static str;     // name for debug and logging purposes only
    fn as_any(&self) -> &dyn Any;               // for downcasting
}

/// A GUI window
pub trait GuiWindow {
    fn draw(&mut self, ctx: &egui::Context, state: &mut CommonState);    // called every frame
    fn retain(&self) -> bool { true }           // override and set to false when done
    fn get_id(&self) -> egui::Id;               // get ID of window
    fn as_any(&self) -> &dyn Any;               // for downcasting
}

pub type GuiWindowLink = Rc<RefCell<dyn GuiWindow>>; 
pub type MenuGroupLink = Rc<RefCell<dyn MenuGroup>>;

