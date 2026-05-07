use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    ShotFired {
        shooter: u32,
        weapon: u16,
        origin: Vec3,
        direction: Vec3,
    },
    DamageApplied {
        target: u32,
        amount: u16,
        attacker: Option<u32>,
    },
    PlayerRespawned {
        player: u32,
        position: Vec3,
    },
}
