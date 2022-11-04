/// # guistate.rs -- window and menu management.
//
//  Top menu bar, and a bottom button bar.
//  Both disappear when not used for a while, for
//  a clean game screen.
//
//  This is the mandatory part of the user interface.
//  Dialogs and menus use this.
//
//  Animats
//  June 2022
//
use std::collections::VecDeque;
use std::rc::{Rc};
use core::cell::RefCell;
use std::any::{Any};
use std::path::PathBuf;
use anyhow::{anyhow, Error};
use simplelog::LevelFilter;
use super::basicintl::Dictionary;
use super::guiutil;
use super::guimenus;
use crate::t;
use crate::{GridSelectParams};
use crate::{MenuGroup};
use super::dialogs::guigrid::GridSelectWindow;
use rend3::{ExtendedAdapterInfo};
use simplelog::{SharedLogger};
/// Configuration
const MESSAGE_SCROLLBACK_LIMIT: usize = 200;   // max scrollback for message window
/// Useful types
pub type SendAny = dyn Any + Send;                  // Can send anything across a channel. Must be boxed, though.
pub type SendAnyBoxed = Box<SendAny>;               // the boxed version
pub type GuiWindowLink = Rc<RefCell<Box<dyn GuiWindow>>>;    // a bit much

/// ***TEMPORARY IMPORTS*** will leave when more code moves outside of libui
use crate::guiwindows::{SystemMode, GuiEvent};


/// Initial values needed to initialize the GUI.
pub struct GuiParams {
    pub version: String,                            // main program version
    pub asset_dir: PathBuf,                         // the asset directory
    pub lang: Dictionary,                           // translation dictionary for chosen language
    pub dark_mode: bool,                            // true if in dark mode
    pub log_level: LevelFilter,                     // logging level
    pub menu_display_secs: u64,                     // (secs) display menus for this long
    pub gpu_info: ExtendedAdapterInfo,              // GPU info
    pub grid_select_params: Vec<GridSelectParams>,  // grid info
}

/// Assets used in displaying the GUI.
#[derive(Default)]
pub struct GuiAssets {
    pub web_icon: egui::TextureId,
}

/// All GUI windows persistent state.
pub struct GuiState {
    //  Data needed in GUI
    pub params: Rc<GuiParams>,                  // starting params
    //  Assets - images, etc.
    pub assets: GuiAssets,
    //  Platform data for context
    pub platform: egui_winit_platform::Platform,
    //  Primary system mode
    system_mode: SystemMode,                // primary operating mode
    //  Selected grid
    pub selected_grid: Option<GridSelectParams>,// params of selected grid, if any
    //  Fixed, reopenable windows.
    pub grid_select_window: GridSelectWindow,   // used at start
    pub message_window: MessageWindow,          // miscellaneous messages ***TEMP***
    pub menu_group_opt: Option<Box<dyn MenuGroup>>, // currently active menu group
    //  Disposable dynamic windows
    temporary_windows: Vec<GuiWindowLink>,
    //  Misc.
    msg_ok: String,                             // translated OK message
    unique_id: usize,                           // unique ID, serial
    last_interaction_time: instant::Instant,    // time of last user 2D interaction
    pub event_send_channel: crossbeam_channel::Sender<SendAnyBoxed>,
    pub event_recv_channel: crossbeam_channel::Receiver<SendAnyBoxed>,
    pub light_mode_visuals: egui::Visuals,          // light mode colors, etc.
    pub dark_mode_visuals: egui::Visuals,           // dark mode colors, etc.
}

impl GuiState {

    /// Usual new
    pub fn new(params: GuiParams, assets: GuiAssets, platform: egui_winit_platform::Platform, 
        event_send_channel: crossbeam_channel::Sender<SendAnyBoxed>, 
        event_recv_channel: crossbeam_channel::Receiver<SendAnyBoxed>) -> GuiState {
        //  Set up base windows.
        let message_window = MessageWindow::new("Messages", t!("window.messages", &params.lang), MESSAGE_SCROLLBACK_LIMIT);
        let grid_select_window = GridSelectWindow::new("Grid select", t!("window.grid_select", &params.lang), &assets, params.grid_select_params.clone());
        //  Set up defaults
        guiutil::set_default_styles(&platform.context());  // set up color and text defaults.
        let light_mode_visuals = egui::Visuals::light();
        let dark_mode_visuals = {
            let mut visuals = egui::Visuals::dark();
            visuals.override_text_color = Some(egui::Color32::WHITE);    // whiter text for dark mode. Usual default is too dim
            visuals
        };
        //  Some common words need translations handy
        let msg_ok =  t!("menu.ok", &params.lang).to_string();
        ////let (event_send_channel, event_recv_channel) = crossbeam_channel::unbounded(); // message channel
        GuiState {
            platform,
            message_window,
            grid_select_window,
            params: Rc::new(params),
            assets,
            temporary_windows: Vec::new(),
            menu_group_opt: None,
            msg_ok,
            unique_id: 0,
            last_interaction_time: instant::Instant::now(),
            system_mode: SystemMode::Start,
            selected_grid: None,
            event_send_channel,
            event_recv_channel,
            light_mode_visuals,
            dark_mode_visuals   
        }
    }
    
    /// Set the currently active menu group. Consumes menu group
    //  So, on a state change, we have to build a new menu group.
    pub fn set_menu_group(&mut self, menu_group: Box<dyn MenuGroup>) {
        self.menu_group_opt = Some(menu_group);
    }
    
    /// Take the currently active menu group out.
    //  Not that useful, 
    pub fn take_menu_group(&mut self) -> Option<Box<dyn MenuGroup>> {
        self.menu_group_opt.take() 
    }
    
    /// Draw all of GUI. Called at beginning of redraw event
    pub fn draw_all(&mut self, window: &winit::window::Window) -> (Vec<egui::ClippedPrimitive>, egui::TexturesDelta) {
        ////self.platform.update_time(data.start_time.elapsed().as_secs_f64());
        self.platform.begin_frame();

        // Insert egui commands here
        let show_menus = self.if_gui_awake();
        let mut inuse = guimenus::draw(self, show_menus); // draws the GUI (BECOMING OBSOLETE)
        //  Draw the active menus.
        /*
        if let Some(menu_group) = &mut self.menu_group_opt {
            inuse |= menu_group.draw(self)
        }
        */
        //  Hokey way to do this
        let taken_menu_group = self.menu_group_opt.take();  // take ownership temporarily to avoid double mutable borrow
        if let Some(mut menu_group) = taken_menu_group {
            inuse |= menu_group.draw(self);
            self.menu_group_opt = Some(menu_group); // put it back
        }
        
        inuse |= is_at_fullscreen_window_top_bottom(window, &self.platform.context()); // check if need to escape from full screen
        if inuse {
            self.wake_up_gui();
        }
        let egui::FullOutput {
            shapes,
            textures_delta,
            platform_output,
            ..
        } = self.platform.end_frame(Some(window));
        if !platform_output.events.is_empty() {
            self.wake_up_gui(); // reset GUI idle time.
            self.message_window.add_line(format!(
                "Platform events: {:?}, {} shapes.",
                platform_output.events,
                shapes.len()
            )); // ***TEMP***
        }
        //  Tesselate and return paint jobs.
        (self.platform.context().tessellate(shapes), textures_delta)    
    }

    /// Draw all live windows
    pub fn draw(&mut self, ctx: &egui::Context) {
        //  Temporary windows
        //  We have to make a list of the windows to do outside "state" to avoid a double mutable borrow.
        let todo_list: Vec<GuiWindowLink> = self.temporary_windows.iter().map(Rc::clone).collect();
        for w in &todo_list { w.borrow_mut().draw(ctx, self) }  // draw all temporaries
        self.temporary_windows.retain(|w| w.borrow().retain());  // keep only live ones
    }
    
    /// General window add
    pub fn add_window(&mut self, window: Box<dyn GuiWindow>) -> Result<(), Error> {
        //  Check for duplicate window
        for w in &self.temporary_windows {
            if w.borrow().get_id() == window.get_id() {
                return Err(anyhow!("Duplicate id for window"));
            }
        }
        self.temporary_windows.push(Rc::new(RefCell::new(window)));
        Ok(())
    }
    	
    /// Get a unique ID, starting from 1.
    pub fn get_unique_id(&mut self) -> egui::Id {
        self.unique_id += 1;                    // serial number increment
        egui::Id::new(self.unique_id)           // unique egui Id
    }
    
    /// Get translations for current language
    pub fn get_lang(&self) -> &Dictionary {
        &self.params.lang
    }
    
    /// Add error message popup.
    //  Handle common errors       
    pub fn add_error_window(&mut self, title: &str, message: &[&str]) {
        //  Create a window with text and an "OK" button.
        let w = TextWindow::new(self.get_unique_id(), title, message, Some(self.msg_ok.as_str()));
        self.add_window(Box::new(w)).expect("Duplicate error msg ID");  // had better not be a duplicate
    }
    
    /// Pop up "unimplemented" message.
    pub fn unimplemented_msg(&mut self) {
        self.add_error_window(t!("menu.unimplemented", self.get_lang()), &[t!("menu.unimplemented", self.get_lang())]);
    }
    
    /// Call this for anything that indicates the GUI should be awakened to show menus.
    pub fn wake_up_gui(&mut self) {
        self.last_interaction_time = instant::Instant::now();
    }
    
    /// Should GUI be shown?
    pub fn if_gui_awake(&self) -> bool {
        self.last_interaction_time.elapsed().as_secs() < self.params.menu_display_secs
    }
    
    pub fn change_mode(&mut self, new_mode: SystemMode) {
        log::info!("System state change: {:?} -> {:?}", self.system_mode, new_mode);
        self.system_mode = new_mode;                    // do state change
    }
    
    pub fn get_mode(&self) -> SystemMode {
        self.system_mode
    }
    
    /// Sends a user event to the event loop.
    //  This ought to use winit events, but we can't do that yet
    //  because of bug https://github.com/BVE-Reborn/rend3/issues/406
    pub fn send_gui_event(&self, event: GuiEvent) -> Result<(), Error> {
        Self::send_gui_event_on_channel(&self.event_send_channel, event)
        ////Ok(self.event_send_channel.send(Box::new(event))?)    // send
        ////let boxed_event: Box<SendAny> = Box::new(event);    // send
        ////Ok(self.event_send_channel.send(boxed_event)?)    // send
    }
    /// Access to channel, mostly for inter-thread sending.
    pub fn get_send_channel(&self) -> &crossbeam_channel::Sender<SendAnyBoxed> {
        &self.event_send_channel        
    }
    /// Send, given channel
    pub fn send_gui_event_on_channel(channel: &crossbeam_channel::Sender<SendAnyBoxed>, event: GuiEvent) -> Result<(), Error> {
        if let Err(_) = channel.send(Box::new(event)) { 
            Err(anyhow!("Error sending GUI event on channel"))  // have to do this because error from send is not sync
        } else {
            Ok(())
        }
    }
    /// Display message in message window
    pub fn add_msg(&mut self, s: String) {
        self.message_window.add_line(s)
    }
/*    
    //  Open replay file dialog, async version.
    pub fn pick_replay_file_async(&mut self, window: &winit::window::Window) {
        pick_replay_file_async(self, window)
    }
*/
}

pub trait GuiWindow {
    fn draw(&mut self, ctx: &egui::Context, state: &mut GuiState);    // called every frame
    fn retain(&self) -> bool { true }           // override and set to false when done
    fn get_id(&self) -> egui::Id;               // get ID of window
}

/// Text window, with noninteractive content.
//  The persistent part
pub struct TextWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    is_open: bool,  // true if open
    message: Vec::<String>, // window text
    dismiss_button: Option::<String>, // for "OK" button if desired
}

impl TextWindow {
    /// Create persistent text window, multiline
    pub fn new(id: egui::Id, title: &str, message: &[&str], dismiss_button: Option::<&str>) -> Self {
        TextWindow {
            id,
            title: title.to_string(),
            message: message.iter().map(|s| s.to_string()).collect(),  // array of String is needed
            is_open: true,  // start open
            dismiss_button: dismiss_button.map(|s| s.to_string())   // to string if present, else none
        }
    }
    
    /// Reopen previously closed window, with old contents.
    pub fn _reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for TextWindow { 
    /// Draw window of text
    fn draw(&mut self, ctx: &egui::Context, _state: &mut GuiState) {
        if self.is_open {
            let mut dismissed = false;          // true if dismiss button pushed
            let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
                .collapsible(false);
            //  Only add window close button in title bar if no "OK" button.
            let window = if self.dismiss_button.is_none() { window.open(&mut self.is_open) } else { window };
            window.show(ctx, |ui| {
                //  Scroll area
                //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
                let total_rows = self.message.len();
                if total_rows == 1 {
                    //  Single-line message, center it.
                    ui.vertical_centered(|ui| {
                        ui.label(self.message[0].as_str());
                    });
                } else {
                    //  Multi-line message, can become scrollable.
                    egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                        for row in row_range {
                            if row >= self.message.len() { break }  // prevent scrolling off end
                            ui.label(self.message[row].as_str());
                        }
                    });
                };
                //  Dismiss button, if present
                if let Some(s) = &self.dismiss_button {
                    ui.vertical_centered(|ui| {
                        if ui.add(egui::Button::new(s)).clicked() {
                            dismissed = true;                       // dismiss
                        }
                    });
                };
            });
            if dismissed { self.is_open = false; } // do here to avoid borrow clash
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

/// A scrolling text message window.
//  The persistent part
pub struct MessageWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    lines: VecDeque<String>,         // the text
}

impl MessageWindow {
    /// Create scrollable message window
    pub fn new(id: &str, title: &str, scrollback_limit: usize) -> Self {
        MessageWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            lines: VecDeque::with_capacity(scrollback_limit),
        }        
    }
    
    /// Add a line of text. Consumes string
    pub fn add_line(&mut self, text: String) {
        self.lines.push_back(text);
    }
    
    /// Draw window of text
    pub fn draw(&self, ctx: &egui::Context, _params: &Rc<GuiParams>) {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id);
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            ////let total_rows = 10;
            egui::ScrollArea::vertical().show_rows(ui, row_height, self.lines.len(), |ui, row_range| {
                for row in row_range {
                    if row >= self.lines.len() { break }
                    let text = &self.lines[row];
                    ui.label(text);
                }
            });
        });
    }
}

/// We're in real trouble and the main GUI isn't running. Modal dialog.
pub fn panic_dialog(title: &str, message: &str) {
    //  Serious errors only. Only use when the main GUI is unavailable at startup
    let _ = rfd::MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_level(rfd::MessageLevel::Error)
        .show();       
}

/// True if cursor is at the top or bottom of the screen in full screen mode.
//  This is how you get the menus back from a totally clean window.
pub fn is_at_fullscreen_window_top_bottom(window: &winit::window::Window, ctx: &egui::Context) -> bool {
    const NEAR_EDGE: f32 = 5.0; // if within this many pixels of top or bottom
                                ////if !window.fullscreen().is_some() { return false; }               // only meaningful for full screen
    let inner_size = window.inner_size(); // sizes of window
    ////let ctx = data.gui_state.platform.context();
    if let Some(pos) = ctx.pointer_interact_pos() {
        // check for pointer at top or bottom of window
        ////println!("pos: {:?}, height: {}", pos, inner_size.height);
        pos.y < NEAR_EDGE || pos.y + NEAR_EDGE > (inner_size.height as f32)
    } else {
        false
    }
}

/// Logging to GUI
//  This is complicated by its having to outlive
//  almost everything else. Even the GUI to which
//  it is logging.
pub struct MessageLogger {
    send_channel: crossbeam_channel::Sender::<SendAnyBoxed>,  // channel for sending messages
    level_filter: LevelFilter,      // errors at this level and above appear for user
    enabled: bool,                  // true if still enabled
}

impl MessageLogger {
    //  Usual new
    pub fn new_logger(level_filter: LevelFilter, send_channel: crossbeam_channel::Sender::<SendAnyBoxed>) -> Box<dyn SharedLogger> {
        Box::new(MessageLogger {
            send_channel,
            level_filter,
            enabled: true               // always on, actually
        })
    }
    
    //  Set log level filter
    pub fn set_level_filter(&mut self, level_filter: LevelFilter) {
        self.level_filter = level_filter;
    }
}

impl log::Log for MessageLogger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        self.enabled
    }
    
    /// Log an error. Filtering has already taken place.
    fn log(&self, record: &log::Record<'_>) {
        // Format for display.
        //  ***NEED TO TRIM LENGTH***
        let s = format!("[{}] ({}): {}",
                record.level(),
                record.target(),
                record.args());
        let event = GuiEvent::LogMessage(s);
        if let Err(e) = GuiState::send_gui_event_on_channel(&self.send_channel, event) {
            println!("Error {}:{} -- {} could not be sent to GUI: {:?}", record.level(), record.target(), record.args(), e);
        }
    }
    
    /// Flush - does nothing.
    fn flush(&self) { }     // no need to flush here
}

//  Logging to GUI
impl SharedLogger for MessageLogger {
    /// Access to level filter
    fn level(&self) -> LevelFilter {
        self.level_filter
    }
    
    /// Access to config
    fn config(&self) -> Option<&simplelog::Config> {
        None                    // for now
    }
    
    /// Return self, boxed. Required.
    fn as_log(self: Box<Self>) -> Box<dyn log::Log> {
        Box::new(self)
    }
}
