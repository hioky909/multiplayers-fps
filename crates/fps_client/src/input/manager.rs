#[derive(Debug, Clone, Copy, Default)]
pub struct InputState {
    pub move_x: f32,
    pub move_y: f32,
    pub shoot: bool,
    pub jump: bool,
}

#[derive(Debug, Default)]
pub struct InputManager {
    state: InputState,
}

impl InputManager {
    pub fn begin_frame(&mut self) {
        self.state.shoot = false;
    }

    pub fn set_move_axis(&mut self, x: f32, y: f32) {
        self.state.move_x = x.clamp(-1.0, 1.0);
        self.state.move_y = y.clamp(-1.0, 1.0);
    }

    pub fn set_jump(&mut self, pressed: bool) {
        self.state.jump = pressed;
    }

    pub fn mark_shoot_pressed(&mut self) {
        self.state.shoot = true;
    }

    pub fn move_axis(&self) -> [f32; 2] {
        [self.state.move_x, self.state.move_y]
    }

    pub fn state(&self) -> InputState {
        self.state
    }
}
