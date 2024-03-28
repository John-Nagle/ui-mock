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
use libui::{ t, GuiWindow, GuiWindowLink, SendAnyBoxed, CommonState, PieMenu };
use egui::Widget;

/// The circular click dialog.
/// The persistent part.
pub struct ClickWindow {
    /// Title of window
    title: String,
    /// Unique ID
    id: egui::Id,        
    /// True if open. Set to false to make it close.
    is_open: bool,
    /// The circular pie menu
    click_menu: PieMenu,
    
}

impl ClickWindow {

    const CLICK_MENU_RADIUS: f32 = 80.0;   // size of pie menu
    const CLICK_MENU_CONTENT: [&'static str;4] = ["menu.pie_menu.sit", "menu.pie_menu.inspect", "", ""];

    /// Open the click window.
    pub fn open_window(state: &mut CommonState) {
        //  Add window if not already open
        let window = Self::new_link("click", t!("menu.world.pie_menu", state.get_lang()), Self::CLICK_MENU_RADIUS, state);
        state.add_window(window).expect("Unable to open click window");     
    }
    
    /// Create click window data areas.
    fn new(
        id: &str,
        title: &str,
        radius: f32,
        state: &mut CommonState,
    ) -> Self {
        ClickWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            is_open: true,
            click_menu: PieMenu::new(
                radius,
                radius/4.0,
                Self::CLICK_MENU_CONTENT.iter().map	(|w| (*w).into()).collect::<Vec<_>>().as_slice(),
                egui::Color32::RED, // line color
                egui::Color32::from_gray(32), // background color
                egui::Color32::GREEN, // hover color
                title,
            ),                

        }
    }   
    /// As link
    fn new_link(id: &str, title: &str, radius: f32, state: &mut CommonState) -> GuiWindowLink {
        Rc::new(RefCell::new(Self::new(id, title, radius, state)))
    }

    /// Reopen previously closed window, with old contents.
    pub fn reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for ClickWindow {
    /// Usual draw function
    fn draw(&mut self, ctx: &egui::Context, _state: &mut CommonState) {
        if self.is_open {
            let mut not_cancelled = true;
            let window = egui::containers::Window::new("")
                .id(self.id)
                .collapsible(false)
                .open(&mut not_cancelled)
                .title_bar(false)
                .fixed_size(egui::Vec2::new(self.click_menu.get_radius()*2.0, self.click_menu.get_radius()*2.0));
            window.show(ctx, |ui| {
                ui.add(&mut self.click_menu);
            });
            if !not_cancelled {
                self.is_open = false;
            } // do here to avoid borrow clash
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
