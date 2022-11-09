// #! libdialog - the application-specific dialogs

mod dialogs;
mod uiinfo;
mod eventswitch;

pub use uiinfo::{UiData, UiInfo, SystemMode, GuiEvent, GridSelectParams};
pub use eventswitch::{handle_gui_event};
