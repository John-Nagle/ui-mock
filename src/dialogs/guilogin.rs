///#  guilogin.rs -- login dialog support
//
//  Animats
//  October 2022
//
use zeroize::{Zeroize, ZeroizeOnDrop};
use super::super::guiwindows::{GuiWindow};
use crate::t;
use crate::{GuiEvent, GuiState,GridSelectParams};
//  Dialog box parameters required for login.
//  The password is zeroized as soon as it can be
//  converted to MD5, and zeroized on drop if 
//  auth is cancelled.
#[derive(Default, ZeroizeOnDrop)]
pub struct LoginDialogInput {
    user_name: String,
    password: String,                   // zeroize this as soon as MD5 is computed
    _auth_token: Option<usize>           // future when 2FA implemented.
}

impl LoginDialogInput {

    /// True if minimum data for login is filled in.
    pub fn is_filled_in(&self) -> bool {
        !self.user_name.trim().is_empty()
    }
    
    pub fn zeroize(&mut self) {
        self.password.zeroize();
    }
}

/// Data needed to do a login.
//  This is passed to the client, and contains the data the server needs for a login.
#[derive(Debug)]
pub struct LoginParams {
    pub grid: GridSelectParams,     // which grid
    pub user_name: String,          // user name
    pub password_md5_opt: Option<md5::Digest>,  // MD5 of the password
    pub auth_token: Option<usize>,  // future when 2FA implemented.
}

/// Login dialog window.
//  The persistent part.
pub struct LoginDialogWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    is_open: bool,  // true if open
    grid: GridSelectParams, // info about grid
    login_dialog_input: LoginDialogInput, // user-provided data needed for login
    remember_username: bool,
    remember_password: bool,
}

impl LoginDialogWindow {
    /// Create persistent text window, multiline
    pub fn new(id: egui::Id, grid: &GridSelectParams) -> Self {
        let title = grid.name.clone();          // title is just grid name for now.
        LoginDialogWindow {
            title,
            id,
            grid: grid.clone(),
            is_open: true,
            login_dialog_input: Default::default(),
            remember_username: false,
            remember_password: true,
        }
    }
    
    /// Reopen previously closed window, with old contents.
    pub fn reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for LoginDialogWindow { 
    /// Draw username/password form.
    //  If username is blank, look up default user name for grid (future feature once prefs are implemented.)
    //  If username is blank, and default name is found, look up password in password storage.
    //  In that case, show actual username in username field and •••• in the password field.
    //  If username is present, but password is blank, look up password MD5 in password storage.
    //  New password is not stored here, 
    fn draw(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        const MIMIMUM_TEXT_BOX_WIDTH: f32 = 200.0;
        if self.is_open {
            let mut accepted = false;          // true if dismiss button pushed
            let mut not_cancelled = true;
            let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
                .collapsible(false)
                .open(&mut not_cancelled)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);
            window.show(ctx, |ui| {             
                egui::Grid::new("login box")
                .min_col_width(MIMIMUM_TEXT_BOX_WIDTH)
                .show(ui, |ui| {
                    ui.horizontal(|ui| { 
                        ui.label(t!("menu.username", &state.params.lang));
                        let _ = ui.add(egui::TextEdit::singleline(&mut self.login_dialog_input.user_name)); 
                    });
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {    // ***MUST CHANGE FOR egui 0.19"***                       
                        ui.checkbox(&mut self.remember_username, t!("menu.remember", &state.params.lang));                           
                    });                    
                    ui.end_row();
                    ui.horizontal(|ui| { 
                        ui.label(t!("menu.password", &state.params.lang));
                        let _ = ui.add(egui::TextEdit::singleline(&mut self.login_dialog_input.password).password(true));
                    });
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {    
                        ui.checkbox(&mut self.remember_password, t!("menu.remember", &state.params.lang));
                    });
                    ui.end_row();
   	            });
                
                ui.vertical_centered(|ui| {
                    let filled_in = self.login_dialog_input.is_filled_in();  // if form filled in
                    if ui.add_enabled(filled_in, egui::Button::new(t!("menu.login", &state.params.lang))).clicked() {
                        let password_md5_opt = if self.login_dialog_input.password.is_empty() {
                            None 
                        } else {
                            Some(md5::compute(&self.login_dialog_input.password.trim()))  // get MD5 of password
                        };
                        self.login_dialog_input.zeroize();              // erase text password in memory
                        accepted = true;                                // dismiss dialog
                        let login_event = GuiEvent::LoginStart(LoginParams {
                            grid: self.grid.clone(),
                            user_name: self.login_dialog_input.user_name.trim().to_string(),
                            password_md5_opt,
                            auth_token: None
                        });
                        let _ = state.send_gui_event(login_event);      // tell main to start the login process
                     }
                });
            });
            if accepted || !not_cancelled { self.is_open = false; } // do here to avoid borrow clash
        }
    }
    
    /// If this is in the dynamic widgets list, drop if retain is false.
    fn retain(&self) -> bool {
        self.is_open
    }
    
    //  Access ID
    fn get_id(&self) -> egui::Id {
        self.id
    }   
}

// ---------------

