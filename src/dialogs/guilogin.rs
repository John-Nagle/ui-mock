///#  guilogin.rs -- login dialog support
//
//  Animats
//  October 2022
//
use std::rc::Rc;
use core::cell::RefCell;
use anyhow::{Error, anyhow};
use zeroize::{Zeroize, ZeroizeOnDrop};
use keyring::{Entry};
use crate::{GuiWindow, GuiWindowLink, Dictionary, CommonState};
use crate::t;
use crate::{GuiEvent, GridSelectParams};  // ***TEMP*** moving out of libui


//  Dialog box parameters required for login.
//  The password is zeroized as soon as it can be
//  converted to MD5, and zeroized on drop if 
//  auth is cancelled.
#[derive(Default, ZeroizeOnDrop)]
struct LoginDialogInput {
    user_name: String,
    password: String,                   // zeroize this as soon as MD5 is computed
    _auth_token: Option<usize>,         // future when 2FA implemented.
    destination: LoginDestination,      // where do we want to go today?
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

/// Where do you want to go today?
#[derive(Default, ZeroizeOnDrop, PartialEq, Eq, Clone, Debug)]
pub enum LoginDestination {
#[default]
    Last,                   // location at last login
    Home,                   // home
    Region(String)          // specified region
}

/// Data needed to do a login.
//  This is passed to the client, and contains the data the server needs for a login.
#[derive(Debug)]
pub struct LoginParams {
    pub grid: GridSelectParams,     // which grid
    pub destination: LoginDestination,  // where on grid
    pub user_name: String,          // user name
    password_md5_opt: Option<String>, // MD5 of the password, including the PASSWORD_PREFIX, ready for login.
    pub auth_token: Option<usize>,  // future when 2FA implemented.
}

impl LoginParams {
    pub const CREDENTIAL_PREFIX: &str = "metaverse";            // credential keys are prefixed with this.
    pub const CRED_TYPE_PASS: &str = "pass";
    pub const CRED_TYPE_TOKEN: &str = "token";
    const PASSWORD_PREFIX: &'static str = "$1$";                // precedes password MD5 in hex. SL convention.

    /// New, without password    
    pub fn new(grid: GridSelectParams, destination: LoginDestination, user_name: String) -> Self {
        LoginParams {
            grid,
            destination,
            user_name,
            password_md5_opt: None,
            auth_token: None
        }
    }
    
    /// Translate special characters and make lower case
    fn translate_special_characters(c: char) -> char {
        match c {
            '.' => '#',
            '/' => '|',
            _ => c
        }
    }
    /// Prep string for use as credential storage key.
    /// Remove all whitespace.
    /// Translate "." and "/" to something else.
    fn prep_string(s: &str) -> String {
        let s: String = s.chars().filter(|c| !c.is_whitespace())
            .map(Self::translate_special_characters).collect();
        s.to_lowercase()            // and force to lower case
    }
    /// Returns the "service" string needed for credential storage.
    /// Format is "PREFIX/SYSTEM/GRID". 
    pub fn get_service(&self, cred_type: &str) -> String {
        format!("{}/{}/{}/{}", Self::CREDENTIAL_PREFIX, Self::prep_string(&self.grid.data.metaverse), Self::prep_string(&self.grid.data.grid), cred_type)
    }
    /// Password as md5
    pub fn get_password_md5(&self) -> Option<String> {
        self.password_md5_opt.clone()
    }
    /// Set password from MD5
    #[allow(clippy::manual_map)]    // avoid excessive use of functional notation when it makes things more confusing
    pub fn set_password_md5(&mut self, digest_opt: Option<md5::Digest>) {        
        self.password_md5_opt = if let Some(digest) = digest_opt {
            Some(format!("{}{:032x}", Self::PASSWORD_PREFIX, digest)) // includes the prefix
        } else {
            None
        }
    }
    
    /// Save password in platform secure storage
    pub fn save_password(&self) -> Result<(), Error> {
        println!("Saving password {:?}", &self.password_md5_opt);    // ***TEMP***
        match &self.password_md5_opt {    
            Some(pass) =>
                Entry::new(&self.get_service(Self::CRED_TYPE_PASS), &Self::prep_string(&self.user_name))
                .set_password(pass).map_err(anyhow::Error::msg),
            None => Err(anyhow!("Attempt to save empty password"))
        }
    }
    /// Fetch password from platform secure storage.
    pub fn fetch_password(&mut self) -> Result<(), Error> {
        let pass = Entry::new(&self.get_service(Self::CRED_TYPE_PASS), &Self::prep_string(&self.user_name))
            .get_password()?;
        self.password_md5_opt = Some(pass);
        Ok(())
    }
    /// Delete password 
    pub fn delete_password(&mut self) -> Result<(), Error> {
        Entry::new(&self.get_service(Self::CRED_TYPE_PASS), &Self::prep_string(&self.user_name))
            .delete_password().map_err(anyhow::Error::msg)
    }
    //  ***NEED TO ADD TOKEN ACCESS FOR 2FA***
}

/// Login dialog window.
//  The persistent part.
pub struct LoginDialogWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    is_open: bool,  // true if open
    grid: GridSelectParams, // info about grid   
    login_dialog_input: LoginDialogInput, // user-provided data needed for login
    remember_password: bool,
}

impl LoginDialogWindow {
    /// Create persistent text window, multiline
    pub fn new(id: egui::Id, grid: &GridSelectParams) -> Self {
        let title = format!("{} -- {}", grid.data.metaverse, grid.data.grid);          // title is just grid name for now.
        LoginDialogWindow {
            title,
            id,
            grid: grid.clone(),
            is_open: true,
            login_dialog_input: Default::default(),
            remember_password: true,
        }
    }
    
    /// As link
    pub fn new_link(id: egui::Id, grid: &GridSelectParams) -> GuiWindowLink {
        Rc::new(RefCell::new(Self::new(id, grid)))
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
    fn draw(&mut self, ctx: &egui::Context, state: &mut CommonState) {
        //  Translated name of destination
        fn destination_name(dest: &LoginDestination, lang: &Dictionary) -> &'static str {
            match dest {
                LoginDestination::Last => t!("menu.last_location", lang),
                LoginDestination::Home => t!("menu.home", lang),
                LoginDestination::Region(_) => t!("menu.region", lang)
            }
        }
                
                
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
                    //  User name
                    ui.horizontal(|ui| { 
                        ui.label(t!("menu.username", &state.params.lang));
                        let _response = ui.add(egui::TextEdit::singleline(&mut self.login_dialog_input.user_name));
                    });
                    ui.end_row();
                    //  Password
                    ui.horizontal(|ui| { 
                        ui.label(t!("menu.password", &state.params.lang));
                        let _ = ui.add(egui::TextEdit::singleline(&mut self.login_dialog_input.password).password(true));
                    });
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {    
                        ui.checkbox(&mut self.remember_password, t!("menu.remember", &state.params.lang));
                    });
                    ui.end_row();
                    //  Destination region
                    //  Dropdown menu. egui calls this a "combo box", but it doesn't have a text input area.
                    ui.horizontal(|ui| { 
                        ui.label(t!("menu.destination_region", &state.params.lang));
                        egui::ComboBox::from_label("")
                            .selected_text(destination_name(&self.login_dialog_input.destination, &state.params.lang))
                            ////format!("{:?}", self.login_dialog_input.destination)) // No translation - fix this
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.login_dialog_input.destination, 
                                    LoginDestination::Last, t!("menu.last_location", &state.params.lang));
                                ui.selectable_value(&mut self.login_dialog_input.destination, 
                                    LoginDestination::Home, t!("menu.home", &state.params.lang));
                                ui.selectable_value(&mut self.login_dialog_input.destination, 
                                    LoginDestination::Region("".to_string()), t!("menu.region", &state.params.lang));
                        })
                    });
                    //  If combo box is "region", allow input of region name
                    if let LoginDestination::Region(ref mut region_name) = &mut self.login_dialog_input.destination {
                         let _ = ui.add(egui::TextEdit::singleline(region_name));
                    };
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
                        /*
                        let mut login_params = LoginParams {
                            grid: self.grid.clone(),
                            destination: self.login_dialog_input.destination.clone(), // which region
                            user_name: self.login_dialog_input.user_name.trim().to_string(),
                            ..Default::default()
                        };
                        */
                        let mut login_params = LoginParams::new(
                            self.grid.clone(),
                            self.login_dialog_input.destination.clone(), // which region
                            self.login_dialog_input.user_name.trim().to_string());
                            
                        if password_md5_opt.is_some() {                 // if a new password was typed in
                            login_params.set_password_md5(password_md5_opt);
                            //  Save newly typed password ***TEMP*** move this to after successful login.
                            if self.remember_password {
                                login_params.save_password().unwrap();  // ***NEED ERROR HANDLING***
                            }
                        } else {                                        // try to get one from storage
                            if login_params.fetch_password().is_err() {   // if no stored password
                                println!("Still need password");        // ***TEMP*** need to make beep sound here.
                                return;
                            }
                        }
                        accepted = true;                                // dismiss dialog
                        println!("Attempting login to {}", login_params.get_service(LoginParams::CRED_TYPE_PASS));                        
                        let _ = state.send_gui_event(GuiEvent::LoginStart(login_params));      // tell main to start the login process
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
