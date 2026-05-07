use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Capsule {
    pub start: Vec3,
    pub end: Vec3,
    pub radius: f32,
}

pub fn aabb_intersects(a: Aabb, b: Aabb) -> bool {
    a.min.x <= b.max.x
        && a.max.x >= b.min.x
        && a.min.y <= b.max.y
        && a.max.y >= b.min.y
        && a.min.z <= b.max.z
        && a.max.z >= b.min.z
}

pub fn point_in_aabb(point: Vec3, aabb: Aabb) -> bool {
    point.cmpge(aabb.min).all() && point.cmple(aabb.max).all()
}
