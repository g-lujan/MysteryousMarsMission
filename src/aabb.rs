use macroquad::prelude::*;

pub struct AABB {
    pub min: Vec3, // Minimum corner (x, y, z)
    pub max: Vec3, // Maximum corner (x, y, z)
}

impl AABB {
    // Create a new AABB from min and max corners
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    // Check if two AABBs are intersecting
    pub fn intersects(&self, other: &AABB) -> bool {
        // Check for overlap on all axes (X, Y, Z)
        self.max.x > other.min.x && self.min.x < other.max.x &&
        self.max.y > other.min.y && self.min.y < other.max.y &&
        self.max.z > other.min.z && self.min.z < other.max.z
    }

    // Check if a point is inside the AABB
    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}
