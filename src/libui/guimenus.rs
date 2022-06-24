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
use crate::{UiAssets};  // ***TEMP***
use super::guiwindows::{GuiState};
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

/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn draw(assets: &UiAssets, state: &mut GuiState, show_menus: bool) -> bool {
    profiling::scope!("Gui");
                           
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
                    ui.menu_button(t!("menu.developer.log_level", state.get_lang()), |ui| {
                        ui.radio_value(&mut state.params.log_level, LevelFilter::Off, t!("menu.log_level.off", state.get_lang()));
                        ui.radio_value(&mut state.params.log_level, LevelFilter::Error, t!("menu.log_level.error", state.get_lang()));
                        ui.radio_value(&mut state.params.log_level, LevelFilter::Warn, t!("menu.log_level.warn", state.get_lang()));
                        ui.radio_value(&mut state.params.log_level, LevelFilter::Info, t!("menu.log_level.info", state.get_lang())); 
                        ui.radio_value(&mut state.params.log_level, LevelFilter::Debug, t!("menu.log_level.debug", state.get_lang()));   
                        ui.radio_value(&mut state.params.log_level, LevelFilter::Trace, t!("menu.log_level.trace", state.get_lang()));                  
                    });                                    
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
                            (*assets).rust_logo,
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

