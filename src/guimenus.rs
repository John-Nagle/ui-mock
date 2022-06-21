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
use super::guiwindows::{MessageWindow};
use sysinfo;

/// Configuration
const HELP_PAGE: &str =
    "https://github.com/John-Nagle/ui-mock#ui-mock---mockup-of-a-game-type-user-interface";
const COPYRIGHT: &str = "© 2022 Animats";

//
/// Avatar->Preferences
pub fn manu_preferences(_ui: &mut Ui, _data: &mut UiData) {
    //  ***MORE***
}

/// Avatar->Quit
pub fn menu_quit(_ui: &mut Ui, data: &mut UiData) {
    data.quit = true;                   // normal exit
}

/// Help->Help
pub fn menu_help_manual(_ui: &mut Ui, _data: &mut UiData) {
    //  Open help page in browser
    webbrowser::open(HELP_PAGE).expect("failed to open URL");   // ***MAKE THIS NON-FATAL***
}
pub fn menu_help_about(_ui: &mut Ui, data: &mut UiData) {
    //  Create window if necessary
    match &mut data.gui_windows.about_window {
        Some(w) => {
            w.is_open = true;      // reopen
        }
        None => {
            //  Generate system information dump
            let if_unknown = |x| if let Some(v) = x { v } else {"unknown".to_string()}; // for Option
            //  Need to create new window
            let mut msgs = Vec::new();
            let version = format!("{}: {}", t!("message.version", data.lang), data.version);
            msgs.push(version.as_str());
            msgs.push(COPYRIGHT);
            use sysinfo::SystemExt;
            let mut sys = sysinfo::System::new_all();           // get system information
            sys.refresh_all();
            let os_info = format!("{}: {}, {}", t!("message.os_version", data.lang), if_unknown(sys.name()), if_unknown(sys.long_os_version()));
            msgs.push(os_info.as_str());
            let system_memory = format!("{}: {:?}", t!("message.system_memory", data.lang), sys.total_memory());
            msgs.push(system_memory.as_str());
            let cpu_count = format!("{}: {}", t!("message.cpu_count", data.lang), sys.cpus().len());
            msgs.push(cpu_count.as_str());
            let about_window = MessageWindow::new("about window", t!("menu.help.about", &data.lang), &msgs);
            data.gui_windows.about_window = Some(about_window);
        }
    }
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
