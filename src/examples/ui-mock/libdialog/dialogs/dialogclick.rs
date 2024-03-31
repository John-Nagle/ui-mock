//! #  dialogclick.rs  -- right-click (usually) menu
//!
//! Displays a pie menu with action options.
//
//  Animats
//  March 2024
//
use std::rc::Rc;
use core::any::Any;
use core::cell::RefCell;
////use crate::GuiAssets;
use libui::{ GuiWindow, GuiWindowLink, SendAnyBoxed, CommonState, PieMenu };

/// The circular click dialog.
/// The persistent part.
pub struct ClickWindow {
    /// Unique ID
    id: egui::Id,        
    /// True if open. Set to false to make it close.
    is_open: bool,
    /// The circular pie menu
    click_menu: PieMenu,
    
}

impl ClickWindow {

    /// Size of pie menu
    const CLICK_MENU_RADIUS: f32 = 100.0;   // size of pie menu
    /// Text of pie menu
    const CLICK_MENU_CONTENT: [&'static str;5] = ["menu.pie_menu.sit", "", "menu.pie_menu.inspect", "", ""];
    /// Background color of pie menu (will be made translucent
    ////const CLICK_MENU_BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 166, 0); // orange
    const CLICK_MENU_BACKGROUND_COLOR: egui::Color32 = egui::Color32::DARK_RED;

    /// Open the click window.
    pub fn open_window(state: &mut CommonState) {
        //  Add window if not already open
        let window = Self::new_link("click", Self::CLICK_MENU_RADIUS, state);
        state.add_window(window).expect("Unable to open click window");     
    }
    
    /// Create click window data areas.
    fn new(
        id: &str,
        radius: f32,
        state: &mut CommonState,
    ) -> Self {
        ClickWindow {
            id: egui::Id::new(id),
            is_open: true,
            click_menu: PieMenu::new(
                radius,
                radius/4.0,
                Self::CLICK_MENU_CONTENT.iter().map	(|w| (state.get_lang().translate(*w)).into()).collect::<Vec<_>>().as_slice(),
                egui::Color32::WHITE,   // text color
                egui::Color32::BLACK, // line color
                Self::CLICK_MENU_BACKGROUND_COLOR, // background color
            ),                
        }
    }   
    /// As link
    fn new_link(id: &str, radius: f32, state: &mut CommonState) -> GuiWindowLink {
        Rc::new(RefCell::new(Self::new(id, radius, state)))
    }

    /// Reopen previously closed window, with old contents.
    pub fn reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for ClickWindow {
    /// Usual draw function
    fn draw(&mut self, ctx: &egui::Context, state: &mut CommonState) {
        let mut click_result_opt = None;
        if self.is_open {
            let mut not_cancelled = true;
            let frame = egui::Frame::none()
            .fill(egui::Color32::TRANSPARENT);
            let window = egui::containers::Window::new("")
                .id(self.id)
                .collapsible(false)
                .open(&mut not_cancelled)
                .title_bar(false)
                .frame(frame)
                .fixed_size(egui::Vec2::new(self.click_menu.get_radius()*2.0, self.click_menu.get_radius()*2.0));
            window.show(ctx, |ui| {
                ui.add(&mut self.click_menu);
            });
            //  Cancel click window when GUI times out.
            not_cancelled = state.if_gui_awake() && self.click_menu.get_click_result().is_none();
            click_result_opt = self.click_menu.get_click_result(); 
            if !not_cancelled {
                self.is_open = false;
            } // do here to avoid borrow clash
        }
        if let Some(click_result) = click_result_opt {
            println!("ClickWindow result: {}", click_result);   // ***TEMP***
        }
    }
/*    
    /// Incoming message event.
    /// We get all GUI events, but only care about one type.
    fn pass_event(&mut self, _state: &mut CommonState, event: &SendAnyBoxed) {
        //  Is this the event we care about, the statistics event?
        if let Some(ev) =  event.downcast_ref::<StatisticsEvent>() {
            //  Push data into plot
            self.frame_time_average.push(ev.frame_time_average);
            self.frame_time_longest.push(ev.frame_time_longest);
        }
    }
*/
    /// If this is in the dynamic widgets list, drop if retain is false.
    fn retain(&self) -> bool {
        self.is_open
    }

    //  Access ID
    fn get_id(&self) -> egui::Id {
        self.id
    }

    /// For downcasting
    fn as_any(&self) -> &dyn Any {
        todo!();    // lifetime problem
        ////self
    }

    /// For downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any {
        todo!();    // lifetime problem
        ////self
    }
}
