use std::collections::VecDeque;

use crate::net::protocol::Snapshot;

#[derive(Debug, Default)]
pub struct SnapshotBuffer {
    snapshots: VecDeque<Snapshot>,
}

impl SnapshotBuffer {
    pub fn push(&mut self, snapshot: Snapshot) {
        self.snapshots.push_back(snapshot);
        while self.snapshots.len() > 64 {
            self.snapshots.pop_front();
        }
    }

    pub fn latest(&self) -> Option<&Snapshot> {
        self.snapshots.back()
    }

    pub fn by_tick(&self, tick: u32) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.tick == tick)
    }
}
