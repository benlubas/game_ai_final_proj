/// This is where I plan to keep all the math, including vector operations I guess
pub mod math {
    use rlbot_lib::rlbot::{Vector3, Rotator};

    pub fn abs_clamp(n: f32, limit: f32) -> f32 {
        if n.abs() > limit {
            return limit * n.signum();
        }
        n
    }

    pub trait Vec3 {
        fn dot(&self, v: &Vector3) -> f32;
        fn dist(&self, v: &Vector3) -> f32;
        fn ground(self) -> Vector3;
        fn local(&self, car_pos: &Vector3, car_orientation: &Rotator) -> f32;
        fn into_vector3(self) -> Vector3;
        fn sub(&self, v: &Vector3) -> Vector3;
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

        /// Not gonna lie, this is from Botimus, and I have no clue what it does right now
        /// It gives some kind of information about this point relative to the car.
        fn local(&self, car_pos: &Vector3, car_orientation: &Rotator) -> f32 {
            self.sub(car_pos).dot(car_orientation)
        }

        /// Operator overloading on a trait is a royal pain in the ass. Thanks rust
        fn sub(&self, v: &Vector3) -> Vector3 {
            Vector3 {
                x: self.x - v.x,
                y: self.y - v.y,
                z: self.z - v.z,
            }
        }

        // kinda a hack to enable operator overloading
        fn into_vector3(self) -> Vector3 {
            self
        }
    }

    // And then this so we can also call `.into()` on them. Just for convenience
    impl Into<Vector3> for &dyn Vec3 {
        fn into(self) -> Vector3 {
            self.into_vector3()
        }
    }

    impl Into<&dyn Vec3> for Rotator {
        fn into(self) -> &'static dyn Vec3{
            return Vector3 { x: self.roll,  z: self.yaw }
        }
    }
}
