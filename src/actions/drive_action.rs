use rlbot_lib::rlbot::{ControllerState, GameTickPacket, Vector3, RenderMessage};

use crate::{utils::{
    arena::Arena,
    math::math::{abs_clamp, forward_vec, up_vec, Vec3}, ActionTickResult,
}, DEFAULT_CAR_ID};

use super::action::{Action, ActionResult};

pub struct DriveAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    pub target_pos: Vector3,
    pub target_speed: f32,
    // backwards: bool, // I'm just going to leave this out for now
    pub drive_on_walls: bool,
}

impl DriveAction {
    pub fn new(target_pos: Vector3, target_speed: f32, drive_on_walls: bool) -> DriveAction {
        DriveAction {
            target_pos,
            target_speed,
            drive_on_walls,
        }
    }
}

// This is blocked on basic drive and flip actions
impl Action for DriveAction {
    fn step(&mut self, tick_packet: GameTickPacket, controller: ControllerState, _dt: f32) -> ActionResult {
        let car = tick_packet
            .clone()
            .players
            .unwrap()
            .get(DEFAULT_CAR_ID)
            .unwrap()
            .physics
            .clone()
            .unwrap();
        let car_location = car.location.clone().unwrap();
        let rotation = car.rotation.clone().unwrap();
        let velocity = car.velocity.clone().unwrap();

        // don't try driving outside the arena
        let mut target = Arena::clamp(&self.target_pos, 100.);

        // smoothly escape goal
        if car_location.y.abs() > Arena::SIZE.y - 50. && car_location.x.abs() < 1000. {
            target = Arena::clamp(&target, 200.);
            target.x = abs_clamp(target.x, 700.);
        }
        if !self.drive_on_walls {
            let seam_radius: f32 = if car_location.y.abs() > Arena::SIZE.y - 100. {
                100.
            } else {
                200.
            };

            if car_location.z > seam_radius {
                // target the point on the ground below the car
                target = target.ground();
            }
        }

        // let local_target = target.local(&location, car.rotation.unwrap());
        // NOTE: This is a place we'd normally want some type of logic for driving backwards
        let bot_to_target_angle =
            (self.target_pos.y - car_location.y).atan2(target.x - car_location.x);
        let bot_front_to_target_angle = bot_to_target_angle - rotation.yaw;

        let mut controller = controller.clone();

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

        let forward_vec = forward_vec(&rotation);
        let forward_vel = velocity.dot(&forward_vec);

        // println!("{forward_vel:?}");
        // # speed controller
        if forward_vel < self.target_speed {
            // println!("moving slower than target speed");
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
            // println!("moving faster than target speed");
            if (forward_vel - self.target_speed) > 400.0 {
                // tap break if we're moving too much faster than the target speed
                controller.throttle = -1.0;
            } else if (forward_vel - self.target_speed) > 100.0 {
                // release the throttle if we're only slightly above target speed
                if up_vec(&rotation).z > 0.85 {
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

        if self.target_pos.dist(&car_location) < 100. {
            return ActionResult::Success;
        }

        return ActionResult::InProgress(ActionTickResult::from(controller));
    }

    fn render(&self) -> Vec<RenderMessage> {
        vec![]
    }

    fn interruptible(&self) -> bool {
        true
    }

    fn kickoff(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        String::from("DriveAction")
    }
}
