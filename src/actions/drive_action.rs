use rlbot_lib::rlbot::{ControllerState, Physics, Vector3};

use crate::utils::{
    arena::Arena,
    math::math::{abs_clamp, forward_vec, up_vec, Vec3},
};

use super::action::{Action, ActionResult};

pub struct DriveAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    car: Option<Box<Physics>>,
    target_pos: Vector3,
    target_speed: f32,
    // backwards: bool, // I'm just going to leave this out for now
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
            let seam_radius: f32 = if location.y.abs() > Arena::SIZE.y - 100. {
                100.
            } else {
                200.
            };

            if location.z > seam_radius {
                // target the point on the ground below the car
                target = target.ground();
            }
        }

        // let local_target = target.local(&location, car.rotation.unwrap());
        // NOTE: This is a place we'd normally want some type of logic for driving backwards
        let bot_to_target_angle =
            (self.target_pos.y - location.y).atan2(self.target_pos.x - location.x);
        let mut bot_front_to_target_angle = bot_to_target_angle - car.rotation.unwrap().yaw;

        let mut controller = ControllerState::default();

        controller.steer = abs_clamp(2.5 * bot_front_to_target_angle, 1.);
        controller.throttle = 1.;

        // TODO: powerslide code
        // self.controls.handbrake = 0
        // if (
        //         abs(phi) > 1.5
        //         and self.car.position[2] < 300
        //         and (ground_distance(self.car, target) < 3500 or abs(self.car.position[0]) > 3500)
        //         and dot(normalize(self.car.velocity), self.car.forward()) > 0.85
        // ):
        //     self.controls.handbrake = 1

        // Speed controller

        let forward_vec = forward_vec(&car.rotation.unwrap());
        let forward_vel = car.velocity.unwrap().dot(&forward_vec);

        // # speed controller
        if forward_vel < self.target_speed {
            controller.throttle = 1.0;
            if self.target_speed > 1400.0
                && forward_vel < 2250.0
                && self.target_speed - forward_vel > 50.0
            {
                controller.boost = true;
            } else {
                controller.boost = false;
            }
        } else {
            if (forward_vel - self.target_speed) > 400.0 {
                // tap break if we're moving too much faster than the target speed
                controller.throttle = -1.0;
            } else if (forward_vel - self.target_speed) > 100.0 {
                // release the throttle if we're only slightly above target speed
                if up_vec(&car.rotation.unwrap()).z > 0.85 {
                    controller.throttle = 0.0;
                } else {
                    // don't release if we're on the wall though
                    controller.throttle = 0.01;
                }
                controller.boost = false;
            }
        }

        // only boost when we're facing the target
        if bot_front_to_target_angle.abs() > 0.3 {
            controller.boost = false;
        }

        if self.target_pos.dist(&location) < 100. {
            return ActionResult::Success
        }

        return ActionResult::InProgress(controller);
    }

    fn render(&self) {
        return;
    }

    fn interruptible(&self) -> bool {
        true
    }

    fn kickoff(&self) -> bool {
        false
    }
}
