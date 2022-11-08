/// #eventswitch.rs -- fan out and handle events.
//
//  The main switch for GUI events.
//
//  As more events are added, more cases will be required here.
//
//  Animats
//  November, 2022
//
use libui::{
    Dictionary, GuiAssets, GuiCommonEvent, GuiParams, GuiState, MessageLogger, SendAnyBoxed,
};
use super::dialogs;
use dialogs::guilogin::{LoginDialogWindow};
use dialogs::menuconnected::{MenuConnected};
use dialogs::guigrid::{GridSelectWindow};

// Workaround for naming problem. Need to rename Ui because of conflict with egui::Ui
use crate::UiData ;
use libui::{t};
use super::uiinfo;
use uiinfo::{UiInfo, SystemMode, GuiEvent, GridSelectParams, pick_replay_file_async};

///  Handle GuiEvent
pub fn handle_gui_event(data: &mut UiData, window: &winit::window::Window, event: &GuiEvent) {
    //  Events can be a GuiEvent or a GuiCommonEvent.
    //  The dynamic typing is to get the definition of GuiEvent out of
    //  libui.
    log::warn!("GuiEvent: {:?}", event);
    match event {
        //  Go to start state.
        GuiEvent::Startup => {
            data.gui_state.app_state.selected_grid = None; // cancel grid selection
            data.gui_state.app_state.change_mode(SystemMode::Startup); // back to starting state
            let grid_select_window = GridSelectWindow::new(
                "Grid select",
                t!(
                    "window.grid_select",
                    &data.gui_state.common_state.params.lang
                ),
                &data.gui_state.common_state.assets,
                data.gui_state.app_state.grid_select_params.clone(),
            );
            let start_menu = dialogs::menustart::MenuStart::new_link(grid_select_window);
            data.gui_state.common_state.set_menu_group(start_menu);
        }
        GuiEvent::OpenReplay(path_buf_opt) => {
            // open a replay file
            match path_buf_opt {
                Some(path_buf) => {
                    println!("Open replay: {:?}", path_buf); // ***TEMP***
                                                             //  ***NEED TO PASS path_buf and grid to startup and actually go*** This is the dummy version
                    data.gui_state.app_state.change_mode(SystemMode::Connected);
                    let connected_menu = MenuConnected::new_link();
                    data.gui_state.common_state.set_menu_group(connected_menu);
                }
                None => {
                    //  User cancelled replay. Back to ground state.
                    data.gui_state
                        .common_state
                        .send_boxed_gui_event(Box::new(GuiEvent::Startup))
                        .unwrap(); // Start up the GUI.
                }
            }
        }
        GuiEvent::LoginTo(grid) => {
            //  Grid has been selected, now try to log in.
            //  Bring up a background with a top menu bar plus a login dialog.

            match data.gui_state.app_state.get_mode() {
                SystemMode::Startup => {
                    data.gui_state.app_state.change_mode(SystemMode::Login); // advance to login state
                    let login_menu = dialogs::menulogin::MenuLogin::new_link();
                    data.gui_state.common_state.set_menu_group(login_menu);
                    let is_file_pick = grid.data.login_url.is_none();
                    data.gui_state.app_state.selected_grid = Some(grid.clone()); // set the selected grid
                    if is_file_pick {
                        //  No grid URL, so this is a replay file selection, not a login.
                        //  File pick is done with the platform's native file picker, asynchronously.
                        //  File pickers are special - they authorize the program to access the file at the system level.
                        pick_replay_file_async(&mut data.gui_state.common_state, window);
                    // use the file picker
                    } else {
                        //  This is a login to a grid. Bring up login dialog window.
                        let id = data.gui_state.common_state.get_unique_id();
                        data.gui_state
                            .common_state
                            .add_window(LoginDialogWindow::new_link(id, &grid))
                            .unwrap();
                    }
                }
                _ => {
                    log::error!(
                        "Login request to {} while in state {:?}",
                        grid.data.metaverse,
                        data.gui_state.app_state.get_mode()
                    );
                }
            }
        }
        GuiEvent::SaveReplay(_path_buf) => {
            // save a replay file
            data.gui_state.common_state.unimplemented_msg(); // ***TEMP***
        }
        GuiEvent::LoginStart(_login_params) => {
            data.gui_state.common_state.unimplemented_msg(); // ***TEMP***
        }
    }
}
