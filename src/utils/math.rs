/// This is where I plan to keep all the math, including vector operations I guess
pub mod math {
    use rlbot_lib::rlbot::{Rotator, Vector3};

    pub fn abs_clamp(n: f32, limit: f32) -> f32 {
        if n.abs() > limit {
            return limit * n.signum();
        }
        n
    }

    /// Convert a rotator to a forward vector
    /// https://stackoverflow.com/questions/1568568/how-to-convert-euler-angles-to-directional-vector
    pub fn forward_vec(rotator: &Rotator) -> Vector3 {
        Vector3 {
            x: rotator.yaw.cos() * rotator.pitch.cos(),
            y: rotator.yaw.sin() * rotator.pitch.cos(),
            z: rotator.pitch.sin(),
        }
    }

    /// convert a rotator to an up vector
    /// taken from wiki https://en.wikipedia.org/wiki/Rotation_matrix -> general 3d rotations
    pub fn up_vec(r: &Rotator) -> Vector3 {
        Vector3 {
            x: r.roll.sin() * r.pitch.sin() * r.yaw.cos() - r.roll.cos() * r.yaw.sin(),
            y: r.roll.sin() * r.pitch.sin() * r.yaw.sin() + r.roll.cos() * r.yaw.cos(),
            z: r.roll.sin() * r.pitch.cos(),
        }
    }

    pub trait Vec3 {
        fn dot(&self, v: &Vector3) -> f32;
        fn dist(&self, v: &Vector3) -> f32;
        fn ground(self) -> Vector3;
        fn sub(&self, v: &Vector3) -> Vector3;
        fn add(&self, v: &Vector3) -> Vector3;
        fn normalize(&self) -> Vector3;
        fn norm(&self) -> f32;
        fn x(&self) -> f32;
        fn y(&self) -> f32;
        fn z(&self) -> f32;
    }

    impl Vec3 for Vector3 {
        /// Calculate the dot product of two vectors
        fn dot(&self, v: &Vector3) -> f32 {
            self.x * v.x + self.y * v.y + self.z * v.z
        }

        /// Calculate the distance from this vector to another
        fn dist(&self, v: &Vector3) -> f32 {
            ((self.x - v.x).powi(2) + (self.y - v.y).powi(2) + (self.z - v.z).powi(2)).sqrt()
        }

        /// Zero the z component of the vector
        fn ground(self) -> Vector3 {
            Vector3 { z: 0., ..self }
        }

        /// Operator overloading on a trait is a royal pain in the ass. Thanks rust
        fn sub(&self, v: &Vector3) -> Vector3 {
            Vector3 {
                x: self.x - v.x,
                y: self.y - v.y,
                z: self.z - v.z,
            }
        }

        fn add(&self, v: &Vector3) -> Vector3 {
            Vector3 {
                x: self.x - v.x,
                y: self.y - v.y,
                z: self.z - v.z,
            }
        }

        /// direction vector of length 1
        fn normalize(&self) -> Vector3 {
            let sum = self.x + self.y + self.z;
            Vector3 {
                x: self.x / sum,
                y: self.y / sum,
                z: self.z / sum,
            }
        }

        /// length of the vector
        fn norm(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }

        // kinda a hack to enable operator overloading
        fn x(&self) -> f32 {
            self.x
        }
        fn z(&self) -> f32 {
            self.z
        }
        fn y(&self) -> f32 {
            self.y
        }
    }

    // And then this so we can also call `.into()` on them. Just for convenience
    impl Into<Vector3> for &dyn Vec3 {
        fn into(self) -> Vector3 {
            Vector3 {
                x: self.x(),
                y: self.y(),
                z: self.z(),
            }
        }
    }
}
