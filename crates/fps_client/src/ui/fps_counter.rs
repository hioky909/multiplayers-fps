#[derive(Debug)]
pub struct FpsCounter {
    smoothed_dt: f32,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            smoothed_dt: 1.0 / 60.0,
        }
    }
}

impl FpsCounter {
    pub fn push_frame(&mut self, dt: f32) {
        let dt = dt.max(0.0001);
        self.smoothed_dt = self.smoothed_dt * 0.9 + dt * 0.1;
    }

    pub fn current_fps(&self) -> u32 {
        (1.0 / self.smoothed_dt) as u32
    }
}
