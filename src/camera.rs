use glam::Vec3;
use macroquad::prelude::*;

const MOVE_SPEED: f32 = 10.5;
const LOOK_SPEED: f32 = 0.2;

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub world_up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, world_up: Vec3) -> Self {
        let yaw: f32 = 1.18;
        let pitch: f32 = 0.0;

        let front = vec3(
            yaw.cos() as f32 * pitch.cos() as f32,
            pitch.sin() as f32,
            yaw.sin() as f32 * pitch.cos() as f32,
        )
        .normalize();

        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();

        Camera {
            position,
            yaw,
            pitch,
            front,
            right,
            up,
            world_up,
        }
    }


    /* Only move forward as gamejam limitation */
    pub fn move_camera(&mut self, delta: f32, direction: Direction) {
        match direction {
            Direction::Forward => {
                self.position.x += 1.0 * MOVE_SPEED * delta;
            },
            _ => {}
        }
    }

    pub fn update_rotation(&mut self, mouse_delta: Vec2, delta: f32) {
        self.yaw += mouse_delta.x * delta * LOOK_SPEED;

        // Restrict the pitch to prevent up/down flipping
        self.pitch += mouse_delta.y * delta * -LOOK_SPEED;
        self.pitch = self.pitch.clamp(-0.5, 0.5); // Limit pitch to a small range for horizontal movement

        self.front = vec3(
            self.yaw.cos() as f32 * self.pitch.cos() as f32,
            0.0, // Fix pitch to avoid vertical movement (y-axis)
            self.yaw.sin() as f32 * self.pitch.cos() as f32,
        )
        .normalize();

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}
