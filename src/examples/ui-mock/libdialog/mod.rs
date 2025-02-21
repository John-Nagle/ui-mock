// #! libdialog - the application-specific dialogs

mod dialogs;
mod eventswitch;
mod uiinfo;
mod screenshot;

pub use dialogs::dialogstats::StatisticsEvent;
pub use eventswitch::handle_gui_event;
pub use uiinfo::{GridSelectParams, GuiEvent, SystemMode, UiAppAssets, UiData, UiInfo};
