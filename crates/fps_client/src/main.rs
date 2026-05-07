mod input;
mod rendering;
mod ui;

use std::time::Instant;

use fps_shared::{
    config::{load_toml, ClientConfig},
    logging::init_logging,
    sim::tick::FixedTimestep,
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging("info");
    let config = load_toml::<ClientConfig>("config/client.toml").unwrap_or_default();
    info!(username = %config.username, server = %config.server_addr, "client boot");

    match rendering::bootstrap::Renderer::bootstrap().await {
        Ok(renderer) => info!(adapter = %renderer.adapter_name, "wgpu bootstrap ready"),
        Err(err) => warn!(%err, "wgpu bootstrap unavailable, continuing in headless mode"),
    }

    let mut input = input::manager::InputManager::default();
    let mut timestep = FixedTimestep::from_hz(128);
    let mut fps_counter = ui::fps_counter::FpsCounter::default();

    let mut last = Instant::now();
    for _ in 0..3 {
        let now = Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        input.begin_frame();
        timestep.push_time(dt);
        let _ = timestep.consume_steps(8);
        fps_counter.push_frame(dt);
    }

    info!(fps = fps_counter.current_fps(), move_axis = ?input.move_axis(), "client skeleton ticked");
    Ok(())
}
