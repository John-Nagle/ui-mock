///#  menuhelp.rs -- the help menu
//
//  This is a submenu called by menuconnected.
//
//  Animats
//  November 2022
//
use libui::{t, TextWindow, CommonState};
use egui::Ui;
/// Configuration
const HELP_PAGE: &str =
    "https://github.com/John-Nagle/ui-mock#ui-mock---mockup-of-a-game-type-user-interface";
const COPYRIGHT: &str = "© 2022 Animats";

/// Help->Help
pub fn menu_help_manual(_ui: &mut Ui, state: &mut CommonState) {
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
pub fn menu_help_about(_ui: &mut Ui, state: &mut CommonState) {
    //  Create window if necessary
    //  Generate system information dump
    let if_unknown = |x| if let Some(v) = x { v } else {"unknown".to_string()}; // for Option
    //  Need to create new window
    let mut msgs = Vec::new();
    let version = format!("{}: {}", t!("message.version", state.get_lang()), state.params.version);
    msgs.push(version.as_str());          
    use sysinfo::SystemExt;
    let mut sys = sysinfo::System::new_all();           // get system information
    //  System info
    sys.refresh_all();
    let os_info = format!("{}: {}, {}", t!("message.os_version", state.get_lang()), if_unknown(sys.name()), if_unknown(sys.long_os_version()));
    msgs.push(os_info.as_str());
    //  CPU info
    let system_memory = format!("{}: {:?}", t!("message.system_memory", state.get_lang()), sys.total_memory());
    msgs.push(system_memory.as_str());
    let cpu_count = format!("{}: {}", t!("message.cpu_count", state.get_lang()), sys.cpus().len());
    msgs.push(cpu_count.as_str());
    //  Graphics subsystem info
    let gpu_name = format!("{}: {:?}, {}", t!("message.gpu_name", state.get_lang()), state.params.gpu_info.device_type, state.params.gpu_info.name);
    msgs.push(gpu_name.as_str());
    let graphics_system = format!("{}: {:?}", t!("message.graphics_system", state.get_lang()), state.params.gpu_info.backend);
    msgs.push(graphics_system.as_str());
    msgs.push(COPYRIGHT);                               // copyright notice
    let about_window = TextWindow::new_link(egui::Id::new("about window"), t!("menu.help.about", state.get_lang()), &msgs, Some(t!("menu.ok", state.get_lang())));
    let _stat = state.add_window(about_window);
}
