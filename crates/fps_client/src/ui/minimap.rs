use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct MinimapIcon {
    pub entity_id: u32,
    pub map_pos: Vec2,
    pub is_enemy: bool,
}
