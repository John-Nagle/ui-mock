//
//  giumenus.rs -- GUI menu actions
//
//  Animats
//  June, 2022
//
//  These are called from the render thread. Do not spend much time here.
//
use crate::UiData;
use crate::t;
use once_cell::sync::OnceCell;
use egui::Ui;
//
/// Avatar->Preferences
pub fn manu_preferences(_ui: &mut Ui, _data: &mut UiData) {
    //  ***MORE***
}

/// Avatar->Quit
pub fn menu_quit(_ui: &mut Ui, data: &mut UiData) {
    data.quit = true;                   // normal exit
}

/// Developer->Open Replay
pub fn menu_open_replay(_ui: &mut Ui, data: &mut UiData) {
    // Open menu entry
    if let Some(path) = rfd::FileDialog::new()
        .set_title(t!("title.open_replay", &data.lang))
        .add_filter("json", &["json"])
        .pick_file()
    {
        let picked_path = Some(path.display().to_string());
        log::warn!("File picked: {}", picked_path.unwrap());
    }
}
