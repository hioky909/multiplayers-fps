use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommand {
    pub sequence: u32,
    pub dt: f32,
    pub move_axis: [f32; 2],
    pub view_yaw_pitch: [f32; 2],
    pub jump: bool,
    pub shoot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub net_id: u32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub health: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub tick: u32,
    pub entities: Vec<EntityState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReliablePayload {
    Login { username: String },
    SpawnGranted { net_id: u32 },
    MatchState { phase: String, round: u16 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    UnreliableInput(UserCommand),
    Snapshot(Snapshot),
    Reliable {
        sequence: u16,
        ack: u16,
        ack_bits: u32,
        payload: ReliablePayload,
    },
    KeepAlive,
}
