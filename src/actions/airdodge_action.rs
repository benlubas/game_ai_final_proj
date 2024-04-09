use rlbot_lib::rlbot::{
    Color, ControllerState, GameTickPacket, RenderMessage, RenderType, Vector3,
};

use crate::{
    utils::{
        math::math::{abs_clamp, forward_vec, Vec3},
        render::render::{line, YELLOW},
        ActionTickResult,
    },
    DEFAULT_CAR_ID,
};

use super::{
    action::{Action, ActionResult},
    jump_action::JumpAction,
};

pub struct AirDodgeAction {
    pub duration: f32,
    pub target: Option<Vector3>,
    pub jump: JumpAction,
    pub jump_finished: bool,
    pub counter: i32,
    pub state_timer: f32,
    car_location: Option<Vector3>,
}

impl AirDodgeAction {
    pub fn new(duration: f32, target: Option<Vector3>) -> AirDodgeAction {
        AirDodgeAction {
            duration,
            target,
            jump: JumpAction::new(duration),
            jump_finished: false,
            counter: 0,
            state_timer: 0.,
            car_location: None,
        }
    }
}

// This is blocked on basic drive and flip actions
impl Action for AirDodgeAction {
    fn step(
        &mut self,
        tick_packet: GameTickPacket,
        controller: ControllerState,
        dt: f32,
    ) -> ActionResult {
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
        self.car_location = Some(car_location.clone());
        let rotation = car.rotation.clone().unwrap();
        let velocity = car.velocity.clone().unwrap();

        // recovery_time = 0.0 if (self.target is None) else 0.4
        let recovery_time = if self.target.is_none() { 0. } else { 0.4 };

        let mut controller = controller.clone();
        if !self.jump_finished {
            match self.jump.step(tick_packet.clone(), controller.clone(), dt) {
                ActionResult::InProgress(ctrl) => {
                    controller = ctrl.input;
                }
                _ => {
                    self.jump_finished = true;
                }
            };
            return ActionResult::InProgress(ActionTickResult::from(controller.to_owned()));
        } else {
            if self.counter == 0 {
                if self.target.is_none() {
                    println!("double jump");
                    // double jump
                    controller.roll = 0.;
                    controller.pitch = 0.;
                    controller.yaw = 0.;
                } else {
                    let target = self.target.clone().unwrap();
                    // air dodge
                    let to_target = car_location.sub(&target).normalize();
                    println!("to_target: {to_target:?}");

                    // self.controls.roll = 0
                    // self.controls.pitch = -target_direction[0]
                    controller.roll = 0.;
                    controller.pitch = -1.;
                    controller.boost = false;

                    // NOTE: this line is probably horribly wrong
                    controller.yaw = (to_target.x / to_target.y).atan();
                    println!("yaw: {}", controller.yaw);
                    // self.controls.yaw = clamp11(sgn(self.car.orientation[2, 2]) * target_direction[1])

                    if to_target.x > 0. && velocity.dot(&forward_vec(&rotation)) > 500. {
                        controller.pitch *= 0.8;
                        controller.yaw = abs_clamp(controller.yaw * 5., 1.);
                    }
                    // if target_local[0] > 0 and dot(self.car.velocity, self.car.forward()) > 500:
                    //    self.controls.pitch = self.controls.pitch * 0.8
                    //    self.controls.yaw = clamp11(self.controls.yaw * 5)
                }
            } else if self.counter == 2 {
                controller.jump = true;
            } else if self.counter >= 4 {
                controller.roll = 0.;
                controller.pitch = 0.;
                controller.yaw = 0.;
                controller.jump = false;
            }
            self.counter += 1;
            self.state_timer += dt;
        };

        // self.finished = self.jump.finished and self.state_timer > recovery_time and self.counter >= 6
        if self.jump_finished && self.state_timer > recovery_time && self.counter >= 6 {
            println!("finished jump");
            return ActionResult::Success;
        }

        return ActionResult::InProgress(ActionTickResult::from(controller.to_owned()));
    }

    fn render(&self) -> Vec<RenderMessage> {
        if let Some(car_location) = self.car_location.clone() {
            if let Some(target) = self.target.clone() {
                return vec![line(car_location, target, YELLOW)];
            }
        }
        vec![]
    }

    fn interruptible(&self) -> bool {
        false
    }

    fn kickoff(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        String::from("AirDodge")
    }
}
