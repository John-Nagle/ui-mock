///  libgui.rs -- The 2D GUI
//
//  Animats
//  June, 2022
//
//  Everything here runs in the main thread.
//  It should not use more than 1% of main thread time.
//  That is checked with Tracy, as being under span "GUI".
//
pub mod basicintl;
pub mod guiactions;
mod guimenus;
pub mod guiwindows;
mod guiutil;

/// The main draw function. Called on every frame.
pub use guimenus::{draw};
/// Utility functions.
pub use guiutil::{load_canned_icon};
