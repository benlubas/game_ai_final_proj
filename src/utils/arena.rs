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

    /// Does this position collide with the wall or the ground? if it does, return the normal of
    /// the collision surface (roughly)
    pub fn collide(pos: &Vector3) -> Option<Vector3> {
        let vec = if pos.x > Arena::SIZE.x {
            Vector3 { x: 1., y: 0., z: 0. }
        } else if pos.x < -Arena::SIZE.x {
            Vector3 { x: -1., y: 0., z: 0. }
        } else if pos.z > Arena::SIZE.y {
            Vector3 { x: 0., y: 0., z: -1. }
        } else if pos.z < 0. {
            Vector3 { x: 0., y: 0., z: 1. }
        } else if pos.y > Arena::SIZE.y {
            Vector3 { x: 0., y: -1., z: 0. }
        } else if pos.y < -Arena::SIZE.y {
            Vector3 { x: 0., y: 1., z: 0. }
        } else {
            return None;
        };

        Some(vec)
    }

    pub fn random_pos(offset: f32) -> Vector3 {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        Vector3 {
            x: rng.gen_range((-Arena::SIZE.x + offset)..(Arena::SIZE.x - offset)),
            y: rng.gen_range((-Arena::SIZE.y + offset)..(Arena::SIZE.y - offset)),
            z: rng.gen_range(offset..(Arena::SIZE.z - offset)),
        }
    }
}
