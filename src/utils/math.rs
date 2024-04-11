/// This is where I plan to keep all the math, including vector operations I guess
pub mod math {
    use std::f32::consts::PI;

    use rlbot_lib::rlbot::{Rotator, Vector3};

    pub fn vec_new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn vec2_new(x: f32, y: f32) -> Vector3 {
        Vector3 { x, y, z: 0. }
    }

    pub fn clamp(n: f32, lower_limit: f32, upper_limit: f32) -> f32 {
        if n > upper_limit {
            upper_limit
        } else if n < lower_limit {
            lower_limit
        } else {
            n
        }
    }

    pub fn abs_clamp(n: f32, limit: f32) -> f32 {
        if n.abs() > limit {
            return limit * n.signum();
        }
        n
    }

    // don't ask me why this works. I don't know
    pub fn rotate(
        axis: &nalgebra::Unit<
            nalgebra::Matrix<
                f32,
                nalgebra::Const<3>,
                nalgebra::Const<1>,
                nalgebra::ArrayStorage<f32, 3, 1>,
            >,
        >,
        rotr: nalgebra::Rotation<f32, 3>,
        rotp: nalgebra::Rotation<f32, 3>,
        roty: nalgebra::Rotation<f32, 3>,
    ) -> nalgebra::Vector3<f32> {
        let axis = rotr.inverse_transform_unit_vector(&axis);
        let axis = rotp.inverse_transform_unit_vector(&axis);
        let axis = roty.inverse_transform_unit_vector(&axis);
        *axis
    }

    pub fn forward_vec(rotator: &Rotator) -> Vector3 {
        let rotr = nalgebra::Rotation::from_euler_angles(0., rotator.roll, 0.);
        let rotp = nalgebra::Rotation::from_euler_angles(-rotator.pitch, 0., 0.);
        let roty = nalgebra::Rotation::from_euler_angles(0., 0., -rotator.yaw + PI / 2.);

        Vector3::from_nalg(rotate(&nalgebra::Vector3::y_axis(), rotr, rotp, roty))
    }

    pub fn up_vec(rotator: &Rotator) -> Vector3 {
        let rotr = nalgebra::Rotation::from_euler_angles(0., rotator.roll, 0.);
        let rotp = nalgebra::Rotation::from_euler_angles(-rotator.pitch, 0., 0.);
        let roty = nalgebra::Rotation::from_euler_angles(0., 0., -rotator.yaw + PI / 2.);

        Vector3::from_nalg(rotate(&nalgebra::Vector3::z_axis(), rotr, rotp, roty))
    }

    pub fn left_vec(rotator: &Rotator) -> Vector3 {
        let rotr = nalgebra::Rotation::from_euler_angles(0., rotator.roll, 0.);
        let rotp = nalgebra::Rotation::from_euler_angles(-rotator.pitch, 0., 0.);
        let roty = nalgebra::Rotation::from_euler_angles(0., 0., -rotator.yaw + PI / 2.);

        Vector3::from_nalg(rotate(&nalgebra::Vector3::x_axis(), rotr, rotp, roty))
    }

    pub fn dir_vecs(rotator: &Rotator) -> Vec<Vector3> {
        let rotr = nalgebra::Rotation::from_euler_angles(0., rotator.roll, 0.);
        let rotp = nalgebra::Rotation::from_euler_angles(-rotator.pitch, 0., 0.);
        let roty = nalgebra::Rotation::from_euler_angles(0., 0., -rotator.yaw + PI / 2.);

        let forward = rotate(&nalgebra::Vector3::y_axis(), rotr, rotp, roty);
        let left = rotate(&nalgebra::Vector3::x_axis(), rotr, rotp, roty);
        let up = rotate(&nalgebra::Vector3::z_axis(), rotr, rotp, roty);

        vec![
            Vector3 {
                x: forward.x,
                y: forward.y,
                z: forward.z,
            },
            Vector3 {
                x: up.x,
                y: up.y,
                z: up.z,
            },
            Vector3 {
                x: left.x,
                y: left.y,
                z: left.z,
            },
        ]
    }

    pub trait Vec3 {
        fn dot(&self, v: &Vector3) -> f32;
        fn cross(&self, v: &Vector3) -> Vector3;
        fn dist(&self, v: &Vector3) -> f32;
        fn ground_dist(&self, v: &Vector3) -> f32;
        fn ground(&self) -> Vector3;
        fn sub(&self, v: &Vector3) -> Vector3;
        fn add(&self, v: &Vector3) -> Vector3;
        fn scale(&self, s: f32) -> Vector3;
        fn normalize(&self) -> Vector3;
        fn norm(&self) -> f32;
        fn angle_between(&self, v: &Vector3) -> f32;
        fn direction(&self, v: &Vector3) -> Vector3;
        fn x(&self) -> f32;
        fn y(&self) -> f32;
        fn z(&self) -> f32;
        fn to_nalg(&self) -> nalgebra::Vector3<f32>;
        fn from_nalg(v: nalgebra::Vector3<f32>) -> Vector3;
        fn up() -> Vector3;
    }

    pub trait Rot3 {
        fn to_nalg(&self) -> nalgebra::Rotation3<f32>;
    }

    impl Vec3 for Vector3 {
        /// Calculate the dot product of two vectors
        fn dot(&self, v: &Vector3) -> f32 {
            self.x * v.x + self.y * v.y + self.z * v.z
        }

        fn cross(&self, v: &Vector3) -> Vector3 {
            Vector3 {
                x: self.y * v.z - self.z * v.y,
                y: self.z * v.x - self.x * v.z,
                z: self.x * v.y - self.y * v.x,
            }
        }

        /// Calculate the distance from this vector to another
        fn dist(&self, v: &Vector3) -> f32 {
            ((self.x - v.x).powi(2) + (self.y - v.y).powi(2) + (self.z - v.z).powi(2)).sqrt()
        }

        /// Zero the z component of the vector
        fn ground(&self) -> Vector3 {
            Vector3 { z: 0., x: self.x, y: self.y }
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
                x: self.x + v.x,
                y: self.y + v.y,
                z: self.z + v.z,
            }
        }

        fn scale(&self, s: f32) -> Vector3 {
            Vector3 {
                x: self.x * s,
                y: self.y * s,
                z: self.z * s,
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

        fn from_nalg(v: nalgebra::Vector3<f32>) -> Vector3 {
            Vector3 {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }

        fn to_nalg(&self) -> nalgebra::Vector3<f32> {
            nalgebra::Vector3::new(self.x, self.y, self.z)
        }

        fn up() -> Vector3 {
            Vector3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }
        }

        fn angle_between(&self, v: &Vector3) -> f32 {
            (self.dot(v) / (self.norm() * v.norm())).acos()
        }

        fn direction(&self, target: &Vector3) -> Vector3 {
            target.sub(self).normalize()
        }

        fn ground_dist(&self, v: &Vector3) -> f32 {
            self.ground().dist(&v.ground())
        }
    }

    impl Rot3 for Rotator {
        fn to_nalg(&self) -> nalgebra::Rotation3<f32> {
            nalgebra::Rotation3::from_euler_angles(self.roll, self.pitch, self.yaw)
        }
    }
}
