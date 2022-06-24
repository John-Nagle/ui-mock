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
use crate::{UiAssets, UiData};
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



/// Update the GUI. Called on each frame.
//  Returns true if the GUI is active and should not disappear.
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.
pub fn draw(assets: &UiAssets, data: &mut UiData, show_menus: bool) -> bool {
    profiling::scope!("Gui");
                           
    // Insert egui commands here
    let ctx = data.platform.context();
    if data.dark_mode {
        ctx.set_visuals(egui::Visuals::dark()); // dark mode if needed
    } else {
        ctx.set_visuals(egui::Visuals::light()); // Switch to light mode
    }
    //  Top menu bar

    if show_menus {
        egui::TopBottomPanel::top("menu_bar").show(&ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button(t!("menu.avatar", &data.lang), |ui| {                                       
                    // Avatar menu
                    if ui.button(t!("menu.avatar.preferences", &data.lang)).clicked() {
                        // Preferences menu entry
                        guiactions::manu_preferences(ui, data);
                    }

                    if ui.button(t!("menu.avatar.quit", &data.lang)).clicked() {
                        guiactions::menu_quit(ui, data);
                    }
                });
                ui.menu_button(t!("menu.comm", &data.lang), |ui| {
                    //  ***MORE***
                    // Help menu
                    if ui.button(t!("menu.unimplemented", &data.lang)).clicked() {
                    }
                });
                ui.menu_button(t!("menu.world", &data.lang), |ui| {
                    //  ***MORE***
                    // Help menu
                    if ui.button(t!("menu.unimplemented", &data.lang)).clicked() {
                    }
                });
                ui.menu_button(t!("menu.content", &data.lang), |ui| {
                    //  ***MORE***
                    // Help menu
                    if ui.button(t!("menu.unimplemented", &data.lang)).clicked() {
                    }
                });        
                ui.menu_button(t!("menu.help", &data.lang), |ui| {
                    // Help menu
                    if ui.button(t!("menu.help", &data.lang)).clicked() {
                        guiactions::menu_help_manual(ui, data);
                    }
                    if ui.button(t!("menu.help.about", &data.lang)).clicked() {
                        // About menu entry
                        guiactions::menu_help_about(ui, data);
                    }
                 });
                 ui.menu_button(t!("menu.developer", &data.lang), |ui| {   
                    //  Log level setting submenu
                    ui.menu_button(t!("menu.developer.log_level", &data.lang), |ui| {
                        ui.radio_value(&mut data.log_level, LevelFilter::Off, t!("menu.log_level.off", &data.lang));
                        ui.radio_value(&mut data.log_level, LevelFilter::Error, t!("menu.log_level.error", &data.lang));
                        ui.radio_value(&mut data.log_level, LevelFilter::Warn, t!("menu.log_level.warn", &data.lang));
                        ui.radio_value(&mut data.log_level, LevelFilter::Info, t!("menu.log_level.info", &data.lang)); 
                        ui.radio_value(&mut data.log_level, LevelFilter::Debug, t!("menu.log_level.debug", &data.lang));   
                        ui.radio_value(&mut data.log_level, LevelFilter::Trace, t!("menu.log_level.trace", &data.lang));                  
                    });                                    
                    //  Replay file menu. Only enabled if compiled with replay feature.
                    //  This is for security of metaverse content.
                    #[cfg (feature="replay")]
                    if ui.button(t!("menu.developer.open_replay", &data.lang)).clicked() {
                        // Open menu entry
                        guiactions::menu_open_replay(ui, data);
                    }
                    #[cfg (feature="replay")]
                    if ui.button(t!("menu.developer.save_replay", &data.lang)).clicked() {
                        // Open menu entry
                        guiactions::menu_open_replay(ui, data);
                    }
                    if ui.button(t!("menu.unimplemented", &data.lang)).clicked() {
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
    data.message_window.new_window(&ctx);   // dummy test window
    data.gui_windows.draw(&ctx); // all the standard windows
    //  Finish
    ctx.is_pointer_over_area() // True if GUI is in use
}

