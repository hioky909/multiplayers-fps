use std::{collections::HashMap, net::SocketAddr, time::Duration};

use fps_shared::{
    config::{load_toml, ServerConfig},
    ecs::{components::NetId, setup::spawn_player},
    logging::init_logging,
    net::protocol::{EntityState, Packet, Snapshot, UserCommand},
    sim::tick::FixedTimestep,
};
use glam::Vec3;
use hecs::World;
use tokio::net::UdpSocket;
use tracing::{debug, info, warn};

#[derive(Debug)]
struct ClientSession {
    net_id: u32,
    last_cmd: Option<UserCommand>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging("info");
    let config = load_toml::<ServerConfig>("config/server.toml").unwrap_or_default();
    run_server(config).await
}

async fn run_server(config: ServerConfig) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(&config.bind_addr).await?;
    info!(%config.bind_addr, tick_rate = config.tick_rate, "server listening");

    let mut world = World::new();
    let mut clients: HashMap<SocketAddr, ClientSession> = HashMap::new();
    let mut timestep = FixedTimestep::from_hz(config.tick_rate);
    let mut tick: u32 = 0;

    let mut recv_buf = [0_u8; 1400];
    let mut interval = tokio::time::interval(Duration::from_secs_f32(timestep.step_secs()));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                timestep.push_time(timestep.step_secs());
                let steps = timestep.consume_steps(4);
                for _ in 0..steps {
                    tick = tick.wrapping_add(1);
                    simulate_step(&mut world, &clients);
                    broadcast_snapshot(&socket, tick, &world, clients.keys().copied().collect()).await?;
                }
            }
            recv = socket.recv_from(&mut recv_buf) => {
                let (len, addr) = recv?;
                if len == 0 {
                    continue;
                }
                match bincode::deserialize::<Packet>(&recv_buf[..len]) {
                    Ok(packet) => handle_packet(&mut world, &mut clients, addr, packet),
                    Err(err) => debug!(%addr, %err, "dropping malformed packet"),
                }
            }
        }
    }
}

fn handle_packet(
    world: &mut World,
    clients: &mut HashMap<SocketAddr, ClientSession>,
    addr: SocketAddr,
    packet: Packet,
) {
    let next_id = (clients.len() + 1) as u32;
    let session = clients.entry(addr).or_insert_with(|| {
        spawn_player(world, next_id, Vec3::new(next_id as f32 * 2.0, 0.0, 0.0));
        info!(%addr, net_id = next_id, "client connected");
        ClientSession {
            net_id: next_id,
            last_cmd: None,
        }
    });

    if let Packet::UnreliableInput(cmd) = packet {
        session.last_cmd = Some(cmd);
    }
}

fn simulate_step(world: &mut World, clients: &HashMap<SocketAddr, ClientSession>) {
    for session in clients.values() {
        let Some(cmd) = &session.last_cmd else {
            continue;
        };
        for (_entity, (id, transform, velocity)) in world.query_mut::<(
            &NetId,
            &mut fps_shared::ecs::components::Transform,
            &mut fps_shared::ecs::components::Velocity,
        )>() {
            if id.0 != session.net_id {
                continue;
            }

            let wish = Vec3::new(cmd.move_axis[0], 0.0, cmd.move_axis[1]).clamp_length_max(1.0);
            velocity.linear = wish * 6.5;
            transform.position += velocity.linear * cmd.dt;
        }
    }
}

async fn broadcast_snapshot(
    socket: &UdpSocket,
    tick: u32,
    world: &World,
    destinations: Vec<SocketAddr>,
) -> anyhow::Result<()> {
    let mut entities = Vec::new();
    for (_entity, (id, transform, velocity, health)) in world
        .query::<(
            &NetId,
            &fps_shared::ecs::components::Transform,
            &fps_shared::ecs::components::Velocity,
            &fps_shared::ecs::components::Health,
        )>()
        .iter()
    {
        entities.push(EntityState {
            net_id: id.0,
            position: transform.position,
            velocity: velocity.linear,
            health: health.current,
        });
    }

    let packet = Packet::Snapshot(Snapshot { tick, entities });
    let payload = bincode::serialize(&packet)?;
    if payload.len() > 1200 {
        warn!(size = payload.len(), "snapshot larger than MTU-safe target");
    }

    for addr in destinations {
        let _ = socket.send_to(&payload, addr).await?;
    }
    Ok(())
}
