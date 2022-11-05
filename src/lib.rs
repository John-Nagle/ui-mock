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
pub mod guiactions; // ***pub is TEMP*** moving elsewhere
mod guimenus;
mod guiwindows;
mod guiutil;
mod guistate;
//  The various dialogs
mod dialogs;

/// The main draw function. Called on every frame.
////pub use guimenus::{draw};
pub use guistate::{GuiParams, GuiAssets, GuiState, MessageLogger, GuiWindow, GuiWindowLink, SendAny, SendAnyBoxed, TextWindow};
pub use guistate::{panic_dialog};
/// Utility functions.
pub use guiutil::{load_canned_icon, load_image, set_default_styles, get_log_file_name, get_executable_name, get_asset_dir, get_cache_dir};
/// Internationalization
pub use basicintl::{Dictionary};
/// Dialogs
pub use dialogs::guilogin::{LoginParams, LoginDialogWindow, LoginDestination};
pub use dialogs::guigrid::{GridSelectParams};

//  ***TEMPORARY*** moving outside of libui
pub use guiwindows::{SystemMode, GuiEvent};
pub use guiwindows::{pick_replay_file_async};

//  Traits
/// A group of menus. Libui user sets what menus are to be shown.
pub trait MenuGroup {
    fn draw(&mut self, state: &mut GuiState) -> bool;    // returns true if menu is in use
}
