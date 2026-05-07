use glam::Vec3;
use hecs::World;

use crate::ecs::components::{Health, NetId, Player, Transform, Velocity, WeaponState};

pub fn spawn_player(world: &mut World, net_id: u32, spawn: Vec3) {
    world.spawn((
        NetId(net_id),
        Player,
        Transform {
            position: spawn,
            yaw_pitch_roll: Vec3::ZERO,
        },
        Velocity { linear: Vec3::ZERO },
        Health {
            current: 100,
            max: 100,
        },
        WeaponState {
            weapon_id: 1,
            ammo_in_mag: 30,
            last_fire_time: -999.0,
        },
    ));
}
