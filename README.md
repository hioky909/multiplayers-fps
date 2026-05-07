# multiplayers-fps

A professional **multiplayer tactical FPS foundation** in Rust, structured as an authoritative-server architecture inspired by CS:GO patterns while remaining practical for a school project.

## 1) Workspace Architecture (Cargo)

```text
multiplayers-fps/
├── Cargo.toml
├── config/
│   ├── client.toml
│   └── server.toml
├── crates/
│   ├── fps_client/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── input/
│   │       │   ├── mod.rs
│   │       │   └── manager.rs
│   │       ├── rendering/
│   │       │   ├── mod.rs
│   │       │   └── bootstrap.rs
│   │       └── ui/
│   │           ├── mod.rs
│   │           ├── fps_counter.rs
│   │           ├── hud.rs
│   │           └── minimap.rs
│   ├── fps_server/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── fps_shared/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── collision.rs
│           ├── config.rs
│           ├── events.rs
│           ├── logging.rs
│           ├── map.rs
│           ├── weapon.rs
│           ├── ecs/
│           │   ├── mod.rs
│           │   ├── components.rs
│           │   └── setup.rs
│           ├── net/
│           │   ├── mod.rs
│           │   ├── protocol.rs
│           │   └── reliability.rs
│           └── sim/
│               ├── mod.rs
│               ├── tick.rs
│               ├── snapshot.rs
│               └── prediction.rs
└── README.md
```

## 2) Client/Server/Shared Separation

- **fps_server**: authoritative simulation, UDP ingest, validation, world state generation, snapshot broadcast.
- **fps_client**: rendering bootstrap, local input capture model, prediction/reconciliation integration points, HUD/FPS/minimap UI model.
- **fps_shared**: protocol contracts, ECS components, deterministic fixed timestep utility, replication models, config/logging utilities.

## 3) Networking Model (UDP + tokio)

### Packet protocol
- `Packet::UnreliableInput(UserCommand)` for high-frequency movement/fire input.
- `Packet::Snapshot(Snapshot)` for authoritative world replication.
- `Packet::Reliable { sequence, ack, ack_bits, payload }` for must-arrive gameplay/session control.
- `Packet::KeepAlive` for idle link continuity.

### Authoritative server (anti-cheat foundation)
The server is the source of truth for movement, hit acceptance, health, and respawn. Clients submit intent only. This reduces speed-hack, teleport, and damage spoofing vectors.

### Prediction + reconciliation
- Client predicts local movement from unacknowledged `UserCommand`s.
- Server snapshots eventually return authoritative state.
- If local predicted position diverges above epsilon, reconciliation corrects and replay can be applied from buffered inputs.

### Interpolation
Remote entities are rendered by interpolating between buffered snapshots (`interpolate_entity`) to smooth jitter and out-of-order packet delivery.

### Packet loss/bandwidth strategy
- Mix unreliable and reliable lanes.
- Keep snapshots compact (delta-compression-ready protocol boundaries).
- Warn when payload exceeds MTU-safe budget.
- Use fixed tick + bounded step catch-up to avoid spiral of death.

## 4) Simulation / Tick Architecture

```text
Client Input -> UDP UserCommand -> Server Input Buffer
                                      |
                                      v
                                Fixed Tick Sim
                                      |
                                      v
                              Authoritative Snapshot
                                      |
                                      v
Client Snapshot Buffer -> Interpolation (remote)
                      -> Reconciliation (local player)
```

## 5) ECS Architecture (hecs)

Data-oriented components:
- `NetId`, `Transform`, `Velocity`, `Health`, `WeaponState`, `Player`

System families:
- Input application (server)
- Movement + collision
- Weapon fire validation
- Damage/respawn
- Snapshot extraction

## 6) Rendering Architecture (wgpu)

`fps_client::rendering::bootstrap::Renderer` initializes adapter/device/queue as the rendering bootstrap boundary. Next production steps:
- frame graph / render passes
- static world batching
- weapon viewmodel pass
- HUD compositing pass
- minimap render target pass

## 7) Input Architecture (winit-ready model)

`InputManager` is separated from rendering and networking and emits deterministic move/shoot/jump state, enabling clean prediction and replay support.

## 8) Weapon, Collision, Maps, UI

- `weapon.rs`: weapon spec and deterministic fire gating.
- `collision.rs`: lightweight primitives (AABB/capsule-ready baseline).
- `map.rs`: map descriptors with minimap/navmesh asset pointers.
- `ui`: FPS counter + HUD state + minimap icon model.

## 9) Threading Model

```text
Main Process
├─ Network Runtime Thread(s) (tokio): UDP IO, serialization
├─ Simulation Tick Thread: fixed-step game logic
└─ Render Thread: interpolation + GPU command submission
```

## 10) Error Handling / Serialization / Logging

- `anyhow` for app-layer error context.
- `serde + bincode` for packet serialization.
- `tracing` + subscriber init for structured logs.

## 11) Scalability Extensions (designed-in)

Architecture is intentionally extension-friendly for:
- bots/AI
- spectator mode
- replay/demo files
- ranking/matchmaking services
- headless dedicated deployment
- mod/plugin registration points

## 12) Performance Guidance

- Keep gameplay deterministic and fixed-step.
- Avoid per-frame allocations in hot paths (reuse buffers, ring buffers).
- Keep snapshot payloads compact and delta-ready.
- Iterate ECS by component locality and narrow query sets.
- Decouple render interpolation from simulation tick.
