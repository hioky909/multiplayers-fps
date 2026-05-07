use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub tick_rate: u32,
    pub max_players: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:27015".to_string(),
            tick_rate: 64,
            max_players: 16,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub username: String,
    pub server_addr: String,
    pub interpolation_delay_ms: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            username: "player".to_string(),
            server_addr: "127.0.0.1:27015".to_string(),
            interpolation_delay_ms: 100,
        }
    }
}

pub fn load_toml<T>(path: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed reading config file: {path}"))?;
    toml::from_str(&raw).with_context(|| format!("failed parsing TOML config file: {path}"))
}
