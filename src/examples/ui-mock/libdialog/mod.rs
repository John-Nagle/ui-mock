// #! libdialog - the application-specific dialogs

mod dialogs;
mod eventswitch;
mod uiinfo;

pub use eventswitch::handle_gui_event;
pub use uiinfo::{GridSelectParams, GuiEvent, SystemMode, UiAppAssets, UiData, UiInfo};
pub use dialogs::dialogstats::{StatisticsEvent};
