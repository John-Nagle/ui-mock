//
//  menuconnected.rs -- Connected to metaverse.
//
//  Menus at top and bottom, and various dialogs.
//
//  Animats
//  November 2022
//
use libui::{GuiState, MenuGroup, MenuGroupLink};
use libui::t;
use egui::{menu, Frame};
use log::{LevelFilter};
use libui::guiactions;  // ***TEMP*** needs to move
use core::cell::RefCell;
use std::rc::Rc;
#[allow(clippy::blocks_in_if_conditions)] // allow excessive nesting, which is the style Egui uses.

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
//
//  The overlay on the main screen. Menus disappear when not used.
//  Cursor to top or bottom of window restores them.
pub struct MenuConnected {
}

impl MenuConnected {
    /// Create new, as trait object
    pub fn new_link() -> MenuGroupLink {
        Rc::new(RefCell::new(MenuConnected{}))                          // create a trait object to dispatch
    }
}

impl MenuGroup for MenuConnected {

    /// Draws the menu set for Login state.
    //  Called on every frame. Do not delay here.
    fn draw(&mut self, state: &mut GuiState) -> bool {                          
        // Insert egui commands here
        let ctx = state.platform.context();
        //  Top menu bar
        let show_menus = state.if_gui_awake();  // show menus only if GUI is needed.
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
                        //  ***MOVE LOG LEVEL TO STATE*** params is read-only.
                        ////state.params.log_level = log_level;     // update log level   
                        #[cfg (feature="replay")]
                        if ui.button(t!("menu.developer.save_replay", state.get_lang())).clicked() {
                            // Open menu entry
                            if ui.button(t!("menu.unimplemented", state.get_lang())).clicked() {}
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
                                state.assets.web_icon,  // placeholder for now
                                egui::Vec2::splat(64.0),
                            )
                            .frame(true),
                        )
                        .clicked()
                    {
                    println!("Clicked on dummy button");
                    }
                    ////ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT; // transparent button background
                });
        }
        //  Non-menu items
        state.message_window.draw(&ctx, &state.params);   // dummy test window
        state.draw(&ctx); // all the standard windows
    //  Finish
        ctx.is_pointer_over_area() // True if GUI is in use
    }
}


