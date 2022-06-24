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
use super::guiwindows::{GuiState, TextWindow};

/// Configuration
const HELP_PAGE: &str =
    "https://github.com/John-Nagle/ui-mock#ui-mock---mockup-of-a-game-type-user-interface";
const COPYRIGHT: &str = "© 2022 Animats";

//
/// Avatar->Preferences
pub fn manu_preferences(_ui: &mut Ui, state: &mut GuiState) {
    //  Unimplemented
    state.add_error_window(t!("menu.unimplemented", state.get_lang()), &[t!("menu.unimplemented", state.get_lang())]);
}

/// Avatar->Quit
pub fn menu_quit(_ui: &mut Ui, state: &mut GuiState) {
    state.quit = true;                   // normal exit
}

/// Help->Help
pub fn menu_help_manual(_ui: &mut Ui, state: &mut GuiState) {
    //  Open help page in browser
    match webbrowser::open(HELP_PAGE) {
        Ok(_) => {},
        Err(e) => {
            //  Popup if trouble
            let errmsg = format!("{:?}",e);
            let messages = [t!("message.web_error", state.get_lang()), errmsg.as_str()];
            state.add_error_window(t!("window.internet_error", state.get_lang()), &messages);
        }
    }
}
pub fn menu_help_about(_ui: &mut Ui, state: &mut GuiState) {
    //  Create window if necessary
    match &mut state.about_window {
        Some(w) => {
            w.reopen();      // reopen window that was already built
        }
        None => {
            //  Generate system information dump
            let if_unknown = |x| if let Some(v) = x { v } else {"unknown".to_string()}; // for Option
            //  Need to create new window
            let mut msgs = Vec::new();
            let version = format!("{}: {}", t!("message.version", state.get_lang()), state.params.version);
            msgs.push(version.as_str());          
            use sysinfo::SystemExt;
            let mut sys = sysinfo::System::new_all();           // get system information
            sys.refresh_all();
            let os_info = format!("{}: {}, {}", t!("message.os_version", state.get_lang()), if_unknown(sys.name()), if_unknown(sys.long_os_version()));
            msgs.push(os_info.as_str());
            let system_memory = format!("{}: {:?}", t!("message.system_memory", state.get_lang()), sys.total_memory());
            msgs.push(system_memory.as_str());
            let cpu_count = format!("{}: {}", t!("message.cpu_count", state.get_lang()), sys.cpus().len());
            msgs.push(cpu_count.as_str());
            msgs.push(COPYRIGHT);                               // copyright notice
            let about_window = TextWindow::new(egui::Id::new("about window"), t!("menu.help.about", state.get_lang()), &msgs, Some(t!("menu.ok", state.get_lang())));
            state.about_window = Some(about_window);
        }
    }
}

/// Developer->Open Replay
#[cfg (feature="replay")]
pub fn menu_open_replay(_ui: &mut Ui, state: &mut GuiState) {
    // Open menu entry
    if let Some(path) = rfd::FileDialog::new()
        .set_title(t!("title.open_replay", state.get_lang()))
        .add_filter("json", &["json"])
        .pick_file()
    {
        let picked_path = Some(path.display().to_string());
        log::warn!("File picked: {}", picked_path.unwrap());
    }
}
