use bevy::prelude::*;

use super::{NotesPattern, PatternReceptor};
use crate::events::CatchNoteEvent;

/// 4点同時押し
#[derive(Component)]
pub struct FullSyncReceptor {
    first_time: f64,
    lane: [bool; 4],
}
impl Default for FullSyncReceptor {
    fn default() -> Self {
        Self {
            first_time: 0.0,
            lane: [false; 4],
        }
    }
}

impl PatternReceptor for FullSyncReceptor {
    fn init(&mut self) {
        self.lane = [false; 4];
    }

    fn is_init(&self) -> bool {
        self.lane.iter().all(|&e| !e)
    }

    fn init_or_defer(&mut self, current_time: f64, _: f32) {
        if (current_time - self.first_time).abs() > 0.1 {
            self.init();
        }
    }

    fn input(&mut self, note_ev: &CatchNoteEvent) {
        // info!("note: {:?}", note_ev);
        let column = note_ev.column;
        let real_sec = note_ev.real_sec;
        if self.is_init() {
            self.first_time = real_sec;
            self.lane[column as usize] = true;
        } else {
            self.lane[column as usize] = true;
        }
    }

    fn achieved(&self) -> Option<super::NotesPattern> {
        self.lane
            .iter()
            .all(|e| *e)
            .then_some(NotesPattern::FullSync)
    }

    fn is_available(&self) -> bool {
        true
    }
}
