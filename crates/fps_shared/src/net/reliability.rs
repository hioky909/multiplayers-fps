#[derive(Debug, Clone, Copy, Default)]
pub struct AckWindow {
    pub latest: u16,
    pub bits: u32,
}

impl AckWindow {
    pub fn record(&mut self, sequence: u16) {
        if sequence > self.latest {
            let shift = (sequence - self.latest) as u32;
            self.bits = (self.bits << shift.min(31)) | 1;
            self.latest = sequence;
        } else {
            let delta = self.latest.wrapping_sub(sequence) as u32;
            if delta < 32 {
                self.bits |= 1 << delta;
            }
        }
    }

    pub fn was_received(&self, sequence: u16) -> bool {
        if sequence == self.latest {
            return true;
        }
        let delta = self.latest.wrapping_sub(sequence) as u32;
        delta < 32 && (self.bits & (1 << delta)) != 0
    }
}
