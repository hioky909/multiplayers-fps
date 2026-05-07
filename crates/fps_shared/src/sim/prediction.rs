use glam::Vec3;

use crate::net::protocol::{EntityState, Snapshot, UserCommand};

const GROUND_PLANE_Y: f32 = 0.0;

#[derive(Debug, Clone)]
pub struct PredictedPlayer {
    pub net_id: u32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub last_applied_input: u32,
}

impl PredictedPlayer {
    pub fn apply_input(&mut self, cmd: &UserCommand, speed: f32) {
        let wish = Vec3::new(cmd.move_axis[0], GROUND_PLANE_Y, cmd.move_axis[1]);
        self.velocity = wish.clamp_length_max(1.0) * speed;
        self.position += self.velocity * cmd.dt;
        self.last_applied_input = cmd.sequence;
    }

    pub fn reconcile(&mut self, snapshot: &Snapshot, epsilon: f32) -> bool {
        let Some(server) = snapshot.entities.iter().find(|e| e.net_id == self.net_id) else {
            return false;
        };
        if self.position.distance(server.position) > epsilon {
            self.position = server.position;
            self.velocity = server.velocity;
            return true;
        }
        false
    }
}

pub fn interpolate_entity(a: &EntityState, b: &EntityState, alpha: f32) -> EntityState {
    EntityState {
        net_id: a.net_id,
        position: a.position.lerp(b.position, alpha),
        velocity: a.velocity.lerp(b.velocity, alpha),
        health: b.health,
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::PredictedPlayer;
    use crate::net::protocol::{EntityState, Snapshot};

    #[test]
    fn reconcile_corrects_large_error() {
        let mut predicted = PredictedPlayer {
            net_id: 7,
            position: Vec3::new(5.0, 0.0, 0.0),
            velocity: Vec3::ZERO,
            last_applied_input: 0,
        };
        let snapshot = Snapshot {
            tick: 3,
            entities: vec![EntityState {
                net_id: 7,
                position: Vec3::new(1.0, 0.0, 0.0),
                velocity: Vec3::ZERO,
                health: 100,
            }],
        };

        assert!(predicted.reconcile(&snapshot, 0.1));
        assert_eq!(predicted.position, Vec3::new(1.0, 0.0, 0.0));
    }
}
