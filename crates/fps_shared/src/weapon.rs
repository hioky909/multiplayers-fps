#[derive(Debug, Clone, Copy)]
pub struct WeaponSpec {
    pub id: u16,
    pub ammo_capacity: u16,
    pub fire_interval_secs: f32,
    pub damage: u16,
}

impl WeaponSpec {
    pub const fn rifle() -> Self {
        Self {
            id: 1,
            ammo_capacity: 30,
            fire_interval_secs: 0.1,
            damage: 35,
        }
    }
}

pub fn can_fire(last_fire_time: f32, now: f32, spec: WeaponSpec, ammo: u16) -> bool {
    ammo > 0 && (now - last_fire_time) >= spec.fire_interval_secs
}
