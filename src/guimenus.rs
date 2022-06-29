//
//  guimenus.rs -- window and menu layout.
//
//  Top menu bar, and a bottom button bar.
//  Both disappear when not used for a while, for
//  a clean game screen.
//
//  Animats
//  June 2022
//
use super::guiwindows::{GuiState, SystemMode};
use super::guiactions;
use egui::{menu, Frame};
use crate::t;
use log::{LevelFilter};

/// Grey background for button area.
//  This really should be a gradient.
const TRANSLUCENT_GREY_ALPHA: u8 = 48;
const TRANSLUCENT_GREY: u8 = 32;
const TRANSLUCENT_GREY_COLOR: u8 =
    ((TRANSLUCENT_GREY_ALPHA as u16 * TRANSLUCENT_GREY as u16) / 256) as u8;
const TRANSLUCENT_GREY_COLOR32: egui::Color32 = egui::Color32::from_rgba_premultiplied(
    TRANSLUCENT_GREY_COLOR,
    TRANSLUCENT_GREY_COLOR,
    TRANSLUCENT_GREY_COLOR,
    TRANSLUCENT_GREY_ALPHA,
);
/*
/// Set dark mode if desired.
pub fn set_dark_mode(ctx: egui::Context, dark_mode: bool) {
    if state.params.dark_mode {
        ctx.set_visuals(egui::Visuals::dark()); // dark mode if needed
    } else {
        ctx.set_visuals(egui::Visuals::light()); // Switch to light mode
    }
}
*/

#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn draw(state: &mut GuiState, show_menus: bool) -> bool {
    profiling::scope!("Gui");
    //  Select appropriate GUI for current mode.
    match state.get_mode() {
        SystemMode::Start => {
            //  Start state. Show available metaverses.
            draw_start(state);
            true
        }
        SystemMode::Connecting => {
            //  Connecting -- display map or something while connecting
            //  ***MORE***
            true
        }
        SystemMode::Connected => {
            //  Connected - fully running.
            draw_connected(state, show_menus)
        }
        SystemMode::Replay => {
            draw_connected(state, show_menus)
        }
        SystemMode::Login => {
            draw_login(state);
            true
        }
        SystemMode::Shutdown => {
            //  Do shutdown stuff
            //  Switch to exit mode
            state.change_mode(SystemMode::Exit);
            true
        }
        SystemMode::Exit => {
            println!("Why are we in exit mode and still running?");
            true
        }
        
    }
}




/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn draw_start(state: &mut GuiState) {                          
    // Insert egui commands here
    let ctx = state.platform.context();
    if state.params.dark_mode {
        ctx.set_visuals(egui::Visuals::dark()); // dark mode if needed
    } else {
        ctx.set_visuals(egui::Visuals::light()); // Switch to light mode
    }
    //  Draw the splash screen with a big set of alternative metaverses.
    //
    egui::TopBottomPanel::top("start_screen").show(&ctx, |ui| {
        state.grid_select_window.new_window(&ctx);   // dummy test window
        state.draw(&ctx); // all the standard windows
    
    });
}

/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn draw_login(state: &mut GuiState) {          
    panic!("Unimplemented"); 
}


#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn draw_connected(state: &mut GuiState, show_menus: bool) -> bool {                          
    // Insert egui commands here
    let ctx = state.platform.context();
    if state.params.dark_mode {
        ctx.set_visuals(egui::Visuals::dark()); // dark mode if needed
    } else {
        ctx.set_visuals(egui::Visuals::light()); // Switch to light mode
    }
    //  Top menu bar

    if show_menus {
        egui::TopBottomPanel::top("menu_bar").show(&ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button(t!("menu.avatar", state.get_lang()), |ui| {                                       
                    // Avatar menu
                    if ui.button(t!("menu.avatar.preferences", state.get_lang())).clicked() {
                        // Preferences menu entry
                        guiactions::manu_preferences(ui, state);
                    }

                    if ui.button(t!("menu.avatar.quit", state.get_lang())).clicked() {
                        guiactions::menu_quit(ui, state);
                    }
                });
                ui.menu_button(t!("menu.comm", state.get_lang()), |ui| {
                    //  ***MORE***
                    // Help menu
                    if ui.button(t!("menu.unimplemented", state.get_lang())).clicked() {
                    }
                });
                ui.menu_button(t!("menu.world", state.get_lang()), |ui| {
                    //  ***MORE***
                    // Help menu
                    if ui.button(t!("menu.unimplemented", state.get_lang())).clicked() {
                    }
                });
                ui.menu_button(t!("menu.content", state.get_lang()), |ui| {
                    //  ***MORE***
                    // Help menu
                    if ui.button(t!("menu.unimplemented", state.get_lang())).clicked() {
                    }
                });        
                ui.menu_button(t!("menu.help", state.get_lang()), |ui| {
                    // Help menu
                    if ui.button(t!("menu.help", state.get_lang())).clicked() {
                        guiactions::menu_help_manual(ui, state);
                    }
                    if ui.button(t!("menu.help.about", state.get_lang())).clicked() {
                        // About menu entry
                        guiactions::menu_help_about(ui, state);
                    }
                 });
                 ui.menu_button(t!("menu.developer", state.get_lang()), |ui| {   
                    //  Log level setting submenu
                    let lang = state.get_lang();
                    let mut log_level = state.params.log_level; // avoid multiple partial borrow of same struct
                    ui.menu_button(t!("menu.developer.log_level", lang), |ui| {
                        ui.radio_value(&mut log_level, LevelFilter::Off, t!("menu.log_level.off", lang));
                        ui.radio_value(&mut log_level, LevelFilter::Error, t!("menu.log_level.error", lang));
                        ui.radio_value(&mut log_level, LevelFilter::Warn, t!("menu.log_level.warn", lang));
                        ui.radio_value(&mut log_level, LevelFilter::Info, t!("menu.log_level.info", lang)); 
                        ui.radio_value(&mut log_level, LevelFilter::Debug, t!("menu.log_level.debug", lang));   
                        ui.radio_value(&mut log_level, LevelFilter::Trace, t!("menu.log_level.trace", lang));                  
                    });
                    state.params.log_level = log_level;     // update log level                                  
                    //  Replay file menu. Only enabled if compiled with replay feature.
                    //  This is for security of metaverse content.
                    #[cfg (feature="replay")]
                    if ui.button(t!("menu.developer.open_replay", state.get_lang())).clicked() {
                        // Open menu entry
                        guiactions::menu_open_replay(ui, state);
                    }
                    #[cfg (feature="replay")]
                    if ui.button(t!("menu.developer.save_replay", state.get_lang())).clicked() {
                        // Open menu entry
                        guiactions::menu_open_replay(ui, state);
                    }
                    if ui.button(t!("menu.unimplemented", state.get_lang())).clicked() {
                    }

                });
            });
        });

        //  Bottom button panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(Frame::none().fill(TRANSLUCENT_GREY_COLOR32))
            .show(&ctx, |ui| {
                ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT; // transparent button background
                if ui
                    .add(
                        egui::widgets::ImageButton::new(
                            state.assets.rust_logo,
                            egui::Vec2::splat(64.0),
                        )
                        .frame(true),
                    )
                    .clicked()
                {
                    println!("Clicked on Rust button");
                }
                ////ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT; // transparent button background
            });
    }
    //  Non-menu items
    state.message_window.new_window(&ctx);   // dummy test window
    state.draw(&ctx); // all the standard windows
    //  Finish
    ctx.is_pointer_over_area() // True if GUI is in use
}

