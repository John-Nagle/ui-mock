//! #  dialogstats.rs  -- statistics dialog
//
//  This is the first screen displayed on startup.
//
//  Animats
//  October 2022
//
use std::rc::Rc;
use core::any::Any;
use crate::GuiAssets;
use libui::{ GuiWindow, SendAnyBoxed, CommonState };

/// Event sent once per second to statistics window to update statistics.
#[derive(Default, Clone, Copy, Debug)]
pub struct StatisticsEvent {
    /// Frames per second, last second
    frame_time_average: f32,
    /// Longest frame time, last second.            
    frame_time_longest: f32,            
    /// UDP queue length, current
    udp_queue_len: u32,
    /// Move task fell behind, count, cumulative
    move_task_lagging: usize,
    /// UDP round trip time (seconds)
    udp_round_trip_time: f32,
    /// UDP Packets sent (cumulative)
    packets_sent: usize,
    /// UDP packets received (cumulative)
    packets_received: usize,
    /// UDP Packets that had to be retransmitted (cumulative)
    packets_retransmitted: usize, 
    /// Incoming UDP packet numbers that were never seen, and thus lost.
    packets_lost: usize,
    /// Asset queue length (current)
    asset_queue_len: usize,
    /// Assets bytes_loaded (cumulative)
    asset_bytes_loaded: usize,
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
    /// Create statistics window data areas.
    pub fn new(
        id: &str,
        title: &str,
    ) -> Self {
        StatisticsWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            is_open: true,
        }
    }
/*    
    /// As link
    pub fn new_link(id: egui::Id, grid: &GridSelectParams) -> GuiWindowLink {
        Rc::new(RefCell::new(Self::new(id, grid)))
    }
*/

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