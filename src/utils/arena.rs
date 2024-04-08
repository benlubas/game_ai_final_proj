use rlbot_lib::rlbot::Vector3;

use super::math::math::abs_clamp;

pub struct Arena {}

impl Arena {
    pub const SIZE: Vector3 = Vector3 {
        x: 4096.,
        y: 5120.,
        z: 2044.,
    };

    pub fn clamp(pos: &Vector3, offset: f32) -> Vector3 {
        return Vector3 {
            x: abs_clamp(pos.x, Arena::SIZE.x - offset),
            y: abs_clamp(pos.y, Arena::SIZE.y - offset),
            z: pos.z,
        };
    }
}
