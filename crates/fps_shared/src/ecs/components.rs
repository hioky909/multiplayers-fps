use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct NetId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub yaw_pitch_roll: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub linear: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: u16,
    pub max: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponState {
    pub weapon_id: u16,
    pub ammo_in_mag: u16,
    pub last_fire_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Player;
