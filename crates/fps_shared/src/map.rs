#[derive(Debug, Clone)]
pub struct MapDescriptor {
    pub id: &'static str,
    pub display_name: &'static str,
    pub navmesh_asset: &'static str,
    pub minimap_texture: &'static str,
}

pub const MAP_POOL: &[MapDescriptor] = &[
    MapDescriptor {
        id: "arena_training",
        display_name: "Arena Training",
        navmesh_asset: "maps/arena_training/navmesh.bin",
        minimap_texture: "maps/arena_training/minimap.png",
    },
    MapDescriptor {
        id: "maze_complex",
        display_name: "Maze Complex",
        navmesh_asset: "maps/maze_complex/navmesh.bin",
        minimap_texture: "maps/maze_complex/minimap.png",
    },
];
