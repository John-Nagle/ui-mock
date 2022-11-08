//
//  guiactions.rs -- GUI menu and button actions
//
//  Animats
//  June, 2022
//
//  These are called from the render thread. Do not spend much time here.
//
use crate::t;
use egui::Ui;
use crate::{CommonState, GuiCommonEvent};

//
/// Avatar->Preferences
pub fn manu_preferences(_ui: &mut Ui, state: &mut CommonState) {
    //  Unimplemented
    state.add_error_window(t!("menu.unimplemented", state.get_lang()), &[t!("menu.unimplemented", state.get_lang())]);
}

/// Avatar->Quit
pub fn menu_quit(_ui: &mut Ui, state: &mut CommonState) {
    let _ = state.send_gui_event(GuiCommonEvent::Shutdown); // tell main loop to quit
}
