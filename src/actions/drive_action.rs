
use rlbot_lib::rlbot::{Physics, Vector3};

use crate::utils::{arena::Arena, math::math::{abs_clamp, Vec3}};

use super::action::{Action, ActionResult};

pub struct DriveAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    car: Option<Box<Physics>>,
    target_pos: Vector3,
    target_speed: f32,
    backwards: bool,
    drive_on_walls: bool,
}

// This is blocked on basic drive and flip actions
impl Action for DriveAction {
    fn step(&self, dt: f32) -> ActionResult {
        let car = self.car.unwrap();
        let location = car.location.unwrap();
        let rotation = car.rotation.unwrap();

        // don't try driving outside the arena
        let mut target = Arena::clamp(self.target_pos, 100.);


        // smoothly escape goal
        if location.y.abs() > Arena::SIZE.y - 50. && location.x.abs() < 1000. {
            target = Arena::clamp(target, 200.);
            target.x = abs_clamp(target.x, 700.);
        }
        if !self.drive_on_walls {
            let seam_radius: f32 = if location.y.abs() > Arena::SIZE.y - 100. { 100. } else { 200. };

            if location.z > seam_radius {
                // target the point on the ground below the car
                target = target.ground();
            }
        }

        let local_target = target.local(&location, car.rotation.unwrap());

        return ActionResult::Failed
    }

    fn render(&self) {
        todo!()
    }

    fn interruptible(&self) -> bool {
        true
    }

    fn kickoff(&self) -> bool {
        false
    }
}
