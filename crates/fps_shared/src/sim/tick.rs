#[derive(Debug, Clone)]
pub struct FixedTimestep {
    step_secs: f32,
    accumulator: f32,
}

impl FixedTimestep {
    pub fn from_hz(hz: u32) -> Self {
        Self {
            step_secs: 1.0 / hz.max(1) as f32,
            accumulator: 0.0,
        }
    }

    pub fn push_time(&mut self, frame_secs: f32) {
        self.accumulator += frame_secs.max(0.0);
    }

    pub fn consume_steps(&mut self, max_steps: usize) -> usize {
        let mut steps = 0;
        while self.accumulator >= self.step_secs && steps < max_steps {
            self.accumulator -= self.step_secs;
            steps += 1;
        }
        steps
    }

    pub fn alpha(&self) -> f32 {
        (self.accumulator / self.step_secs).clamp(0.0, 1.0)
    }

    pub fn step_secs(&self) -> f32 {
        self.step_secs
    }
}

#[cfg(test)]
mod tests {
    use super::FixedTimestep;

    #[test]
    fn consumes_expected_steps() {
        let mut stepper = FixedTimestep::from_hz(20);
        stepper.push_time(0.26);
        assert_eq!(stepper.consume_steps(10), 5);
    }
}
