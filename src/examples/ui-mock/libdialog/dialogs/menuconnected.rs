//
//  menuconnected.rs -- Connected to metaverse.
//
//  Menus at top and bottom, and various dialogs.
//
//  Animats
//  November 2022
//
use super::menuavatar;
use super::menuhelp::{menu_help_about, menu_help_manual}; // submenus
use crate::UiAppAssets;
use core::any::Any;
use core::cell::RefCell;
use egui::{menu, Frame};
use libui::t;
use libui::{CommonState, MenuGroup, MenuGroupLink, NavArrows, NavAction};
use log::LevelFilter;
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
    move_arrows: NavArrows,
    rot_arrows: NavArrows,
}

impl MenuConnected {
    /// Create new, as trait object. Provide needed graphical assets.
    pub fn new_link(assets: &UiAppAssets) -> MenuGroupLink {
        //  Two four-way controls with a center reset button
        let button_size = egui::Vec2::splat(64.0);
        Rc::new(RefCell::new(MenuConnected {
            move_arrows: NavArrows::new(
                (assets.move_arrows_icon, button_size),
                (assets.pressed_arrow_icon, button_size),
                (assets.pressed_button_icon, button_size),
                8.0,
                "Move camera",
            ),
            rot_arrows: NavArrows::new(
                (assets.rot_arrows_icon, button_size),
                (assets.pressed_arrow_icon, button_size),
                (assets.pressed_button_icon, button_size),
                8.0,
                "Aim camera"
            ),
        })) // create a trait object to dispatch
    }
}

impl MenuGroup for MenuConnected {
    /// Draws the menu set for Login state.
    //  Called on every frame. Do not delay here.
    fn draw(&mut self, state: &mut CommonState) -> bool {
        // Insert egui commands here
        let ctx = state.context.clone();
        //  Top menu bar
        let show_menus = state.if_gui_awake(); // show menus only if GUI is needed.
        if show_menus {
            egui::TopBottomPanel::top("menu_bar").show(&ctx, |ui| {
                menu::bar(ui, |ui| {
                    ui.menu_button(t!("menu.avatar", state.get_lang()), |ui| {
                        // Avatar menu
                        if ui
                            .button(t!("menu.avatar.preferences", state.get_lang()))
                            .clicked()
                        {
                            // Preferences menu entry
                            menuavatar::menu_preferences(ui, state);
                        }

                        if ui
                            .button(t!("menu.avatar.quit", state.get_lang()))
                            .clicked()
                        {
                            menuavatar::menu_quit(ui, state);
                        }
                    });
                    ui.menu_button(t!("menu.comm", state.get_lang()), |ui| {
                        //  ***MORE***
                        // Help menu
                        if ui
                            .button(t!("menu.unimplemented", state.get_lang()))
                            .clicked()
                        {}
                    });
                    ui.menu_button(t!("menu.world", state.get_lang()), |ui| {
                        //  ***MORE***
                        // Help menu
                        if ui
                            .button(t!("menu.unimplemented", state.get_lang()))
                            .clicked()
                        {}
                    });
                    ui.menu_button(t!("menu.content", state.get_lang()), |ui| {
                        //  ***MORE***
                        // Help menu
                        if ui
                            .button(t!("menu.unimplemented", state.get_lang()))
                            .clicked()
                        {}
                    });
                    ui.menu_button(t!("menu.help", state.get_lang()), |ui| {
                        // Help menu
                        if ui.button(t!("menu.help", state.get_lang())).clicked() {
                            menu_help_manual(ui, state);
                        }
                        if ui.button(t!("menu.help.about", state.get_lang())).clicked() {
                            // About menu entry
                            menu_help_about(ui, state);
                        }
                    });
                    ui.menu_button(t!("menu.developer", state.get_lang()), |ui| {
                        //  Log level setting submenu
                        let lang = state.get_lang();
                        let mut log_level = state.params.log_level; // avoid multiple partial borrow of same struct
                        ui.menu_button(t!("menu.developer.log_level", lang), |ui| {
                            ui.radio_value(
                                &mut log_level,
                                LevelFilter::Off,
                                t!("menu.log_level.off", lang),
                            );
                            ui.radio_value(
                                &mut log_level,
                                LevelFilter::Error,
                                t!("menu.log_level.error", lang),
                            );
                            ui.radio_value(
                                &mut log_level,
                                LevelFilter::Warn,
                                t!("menu.log_level.warn", lang),
                            );
                            ui.radio_value(
                                &mut log_level,
                                LevelFilter::Info,
                                t!("menu.log_level.info", lang),
                            );
                            ui.radio_value(
                                &mut log_level,
                                LevelFilter::Debug,
                                t!("menu.log_level.debug", lang),
                            );
                            ui.radio_value(
                                &mut log_level,
                                LevelFilter::Trace,
                                t!("menu.log_level.trace", lang),
                            );
                        });
                        //  ***MOVE LOG LEVEL TO STATE*** params is read-only.
                        ////state.params.log_level = log_level;     // update log level
                        #[cfg(feature = "replay")]
                        if ui
                            .button(t!("menu.developer.save_replay", state.get_lang()))
                            .clicked()
                        {
                            // Open menu entry
                            if ui
                                .button(t!("menu.unimplemented", state.get_lang()))
                                .clicked()
                            {}
                        }
                    });
                });
            });

            //  Bottom button panel
            egui::TopBottomPanel::bottom("bottom_panel")
                .frame(Frame::none().fill(TRANSLUCENT_GREY_COLOR32))
                .show(&ctx, |ui| {
                    ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT; // transparent button background
                    ui.horizontal(|ui| {
                        let response = ui.add(&mut self.rot_arrows);
                        let action = self.rot_arrows.decode_response(&response);
                        if action != NavAction::None {
                            state.add_msg(format!("Rotation arrows: {:?}", action));
                        }
                        let response = ui.add(&mut self.move_arrows);
                        let action = self.move_arrows.decode_response(&response);
                        if action != NavAction::None {
                            state.add_msg(format!("Move arrows: {:?}", action));
                        }
                    })
                });
        }
        //  Non-menu items
        state.message_window.draw(&ctx, &state.params); // dummy test window
        state.draw(&ctx); // all the standard windows
                          //  Finish
        ctx.is_pointer_over_area() // True if GUI is in use
    }

    /// Ident for debug purposes
    fn get_name(&self) -> &'static str {
        "Connected"
    }

    /// For downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// For downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
