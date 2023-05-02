//! #  dialogstats.rs  -- statistics dialog
//!
//! Displays useful performance graphs.
//
//  Animats
//  May 2023
//
use std::rc::Rc;
use core::any::Any;
use core::cell::RefCell;
use crate::GuiAssets;
use libui::{ t, GuiWindow, GuiWindowLink, SendAnyBoxed, CommonState };

/// Event sent once per second to statistics window to update statistics.
/// These represent most of the potential bottlenecks.
#[derive(Default, Clone, Copy, Debug)]
pub struct StatisticsEvent {
    /// Frames per second, last second
    pub frame_time_average: f32,
    /// Longest frame time, last second.            
    pub frame_time_longest: f32,            
    /// UDP queue length, current
    pub udp_queue_len: u32,
    /// Move task fell behind, count, cumulative
    pub move_task_lagging: usize,
    /// UDP round trip time (seconds)
    pub udp_round_trip_time: f32,
    /// UDP Packets sent (cumulative)
    pub packets_sent: usize,
    /// UDP packets received (cumulative)
    pub packets_received: usize,
    /// UDP Packets that had to be retransmitted (cumulative)
    pub packets_retransmitted: usize, 
    /// Incoming UDP packet numbers that were never seen, and thus lost.
    pub packets_lost: usize,
    /// Asset queue length (current)
    pub asset_queue_len: usize,
    /// Assets bytes_loaded (cumulative)
    pub asset_bytes_loaded: usize,
}

impl StatisticsEvent {
    /// For downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The performance statistics window.
/// The persistent part.
pub struct StatisticsWindow {
    /// Title of window
    title: String,
    /// Unique ID
    id: egui::Id,        
    /// True if open. Set to false to make it close.
    is_open: bool,
}

impl StatisticsWindow {

    /// Open the statistics window.
    pub fn open_window(state: &mut CommonState) {
        //  Add window if not already open
        let window = Self::new_link("stats", t!("Performance statistics", state.get_lang()));
        state.add_window(window).expect("Unable to open statistics window");     
    }
    
    /// Create statistics window data areas.
    fn new(
        id: &str,
        title: &str,
    ) -> Self {
        StatisticsWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            is_open: true,
        }
    }   
    /// As link
    fn new_link(id: &str, title: &str) -> GuiWindowLink {
        Rc::new(RefCell::new(Self::new(id, title)))
    }

    /// Reopen previously closed window, with old contents.
    pub fn reopen(&mut self) {
        self.is_open = true;
    }
}

impl GuiWindow for StatisticsWindow {
    /// Usual draw function
    fn draw(&mut self, ctx: &egui::Context, state: &mut CommonState) {
        const MIMIMUM_STATISTICS_BOX_WIDTH: f32 = 100.0;
        if self.is_open {
            let mut not_cancelled = true;
            let window = egui::containers::Window::new(self.title.as_str())
                .id(self.id)
                .collapsible(true)
                .open(&mut not_cancelled);
            window.show(ctx, |ui| {
                egui::Grid::new("statistics box")
                    .min_col_width(MIMIMUM_STATISTICS_BOX_WIDTH)
                    .show(ui, |ui| {
                        ui.label("FPS");    // ***TEMP***
                        ui.end_row();
                        ui.label("Sinewave");   // ***TEMP***
                        ui.end_row();
                });
            });
            if !not_cancelled {
                self.is_open = false;
            } // do here to avoid borrow clash
        }
    }
    
    /// Incoming message event.
    /// We get all GUI events, but only care about one type.
    fn pass_event(&mut self, _state: &mut CommonState, event: &SendAnyBoxed) {
        //  Is this the event we care about, the statistics event?
        if let Some(ev) =  event.downcast_ref::<StatisticsEvent>() {
        //  ***MORE***
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

    /// For downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// For downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
