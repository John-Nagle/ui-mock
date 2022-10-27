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
mod guiactions;
mod guimenus;
mod guiwindows;
mod guiutil;
//  The various dialogs
mod dialogs;

/// The main draw function. Called on every frame.
////pub use guimenus::{draw};
pub use guiwindows::{GuiParams, GuiAssets, GuiState, GuiEvent, SystemMode, MessageLogger};
pub use guiwindows::{panic_dialog};
/// Utility functions.
pub use guiutil::{load_canned_icon, set_default_styles, get_log_file_name, get_executable_name, get_asset_dir, get_cache_dir};
/// Internationalization
pub use basicintl::{Dictionary};
/// Dialogs
pub use dialogs::guilogin::{LoginParams, LoginDialogWindow};
pub use dialogs::guigrid::{GridSelectParams};
