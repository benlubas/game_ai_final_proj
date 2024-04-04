
use rlbot_lib::rlbot::{Physics, ControllerState};

use super::action::{Action, ActionResult};

pub struct BasicKickoffAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    current_time: f32,
    phase: i32,
    car: Option<Box<Physics>>,
}

impl BasicKickoffAction {
    pub fn defaults() -> BasicKickoffAction {
        return BasicKickoffAction {
            current_time: 0.,
            phase: 1,
            car: None,
        };
    }
}

// This is blocked on basic drive and flip actions
impl Action for BasicKickoffAction {
    fn step(&self, dt: f32) -> ActionResult {
        let controller = ControllerState::default();
        if let Some(car) = self.car {
            if self.phase == 1 {
                let speed_threshold = if car.location.unwrap().x.abs() < 100. { 1550 } else { 1400 };
                // if car.velocity.unwrap() norm(car.velocity) > speed_threshold:
                //     self.phase = 2
                //     self.action = AirDodge(car, 0.1, car.position + car.velocity)
                // TODO: more controller
                return ActionResult::InProgress(controller)
            }
            //
            // if self.phase == 2:
            // self.action.controls.boost = self.action.state_timer < 0.1
            //
            // if car.on_ground and self.action.finished:
            //     self.action = self.drive
            //     self.phase = 3
            //
            //     if self.phase == 3:
            //     if distance(car, vec3(0, 0, 93)) < norm(car.velocity) * 0.3:
            //         self.phase = 4
            //         self.action = AirDodge(car, 0.1, self.info.ball.position)
            //
            //             self.counter_fake_kickoff()
            //
            //         if self.phase == 4:
            //         if self.action.finished:
            //         self.finished = True
            //
            //             super().step(dt)
        }
        return ActionResult::InProgress(controller)
    }

    fn render(&self) {}

    fn interruptible(&self) -> bool {
        false
    }

    fn kickoff(&self) -> bool {
        true
    }
}
