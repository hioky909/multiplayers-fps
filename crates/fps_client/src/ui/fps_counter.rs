const MIN_FRAME_TIME: f32 = 0.0001;
const EMA_PREVIOUS_WEIGHT: f32 = 0.9;
const EMA_CURRENT_WEIGHT: f32 = 0.1;

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
        let dt = dt.max(MIN_FRAME_TIME);
        self.smoothed_dt = self.smoothed_dt * EMA_PREVIOUS_WEIGHT + dt * EMA_CURRENT_WEIGHT;
    }

    pub fn current_fps(&self) -> u32 {
        (1.0 / self.smoothed_dt) as u32
    }
}
