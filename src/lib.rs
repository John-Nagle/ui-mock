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

/// The main draw function. Called on every frame.
pub use guimenus::{draw};
pub use guiwindows::{GuiParams, GuiAssets, GuiState, GuiEvent, SystemMode, GridSelectParams};
/// Utility functions.
pub use guiutil::{load_canned_icon, set_default_styles};
/// Internationalization
pub use basicintl::{Dictionary};