use rlbot_lib::rlbot::{ControllerState, GameTickPacket, Physics, Vector3};

use crate::{utils::math::math::Vec3, DEFAULT_CAR_ID};

use super::{
    action::{Action, ActionResult},
    airdodge_action::AirDodgeAction,
    drive_action::DriveAction,
    jump_action::JumpAction,
};

pub struct BasicKickoffAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    current_time: f32,
    phase: i32,
    action: Option<Box<dyn Action>>,
    action_state: Option<ActionResult>,
}

impl BasicKickoffAction {
    pub fn new() -> BasicKickoffAction {
        BasicKickoffAction {
            ..BasicKickoffAction::defaults()
        }
    }
    pub fn defaults() -> BasicKickoffAction {
        return BasicKickoffAction {
            current_time: 0.,
            phase: 0,
            action: None,
            action_state: None,
        };
    }
}

// This is blocked on basic drive and flip actions
impl Action for BasicKickoffAction {
    fn step(&mut self, tick_packet: GameTickPacket, controller: ControllerState, dt: f32) -> ActionResult {
        let mut controller = controller.clone();
        let players = tick_packet.clone().players.clone().unwrap();
        let car = players.get(DEFAULT_CAR_ID).clone().unwrap();
        let car_phys = car.physics.clone().unwrap();
        let car_location = car_phys.location.clone().unwrap();
        let car_velocity = car_phys.velocity.clone().unwrap();
        let ball_location = tick_packet.clone().ball.unwrap().physics.unwrap().location.unwrap();

        println!("Kickoff action pase: {}", self.phase);

        if self.phase == 0 {
            self.action = Some(Box::new(DriveAction::new(ball_location.clone(), 2300., false)));
            self.phase = 1;
        }

        if self.phase == 1 {
            let speed_threshold = if car_location.x.abs() < 100. {
                1550.0
            } else {
                1400.0
            };
            // if norm(car.velocity) > speed_threshold:
            //     self.phase = 2
            //     self.action = AirDodge(car, 0.1, car.position + car.velocity)
            if car_velocity.norm() > speed_threshold {
                self.phase = 2;
                self.action = Some(Box::new(AirDodgeAction {
                    duration: 0.1,
                    target: Some(car_location.add(&ball_location)),
                    jump: JumpAction::new(0.1),
                    jump_finished: false,
                    counter: 0,
                    state_timer: 0.,
                }))
            }
        }
        if self.phase == 2 {
            // self.action.controls.boost = self.action.state_timer < 0.1
            controller.boost = self.current_time < 0.1;

            // if car.on_ground and self.action.finished:
            if car.hasWheelContact && matches!(self.action_state.as_ref().unwrap_or(&ActionResult::Failed), &ActionResult::Success) {
            //     self.action = self.drive
            //     self.phase = 3
                self.action = Some(Box::new(DriveAction {
                    target_pos: ball_location.clone(),
                    target_speed: 2300.,
                    drive_on_walls: false,
                }));
                self.phase = 3;
            }
        }
        if self.phase == 3 {
            if car_location.dist(&Vector3 { x: 0., y: 0., z: 93. }) < car_velocity.norm() * 0.3 {
                self.phase = 4;
                self.action = Some(Box::new(AirDodgeAction::new(0.1, Some(ball_location))));

                // TODO: counter fake kickoff
            }
            //     if distance(car, vec3(0, 0, 93)) < norm(car.velocity) * 0.3:
            //         self.phase = 4
            //         self.action = AirDodge(car, 0.1, self.info.ball.position)
            //
            //             self.counter_fake_kickoff()
        }

        // tick the action:
        // match &self.action {
        //     Some(action) => {
        //         action
        //     },
        //     None => todo!(),
        // };

        if let Some(action) = self.action.as_mut() {
            println!("ticking action: {}", action.name());
            match action.step(tick_packet.clone(), controller.clone(), dt) {
                ActionResult::Success => {
                    if self.phase == 4 {
                        return ActionResult::Success
                    }
                },
                ActionResult::Failed => return ActionResult::Failed,
                ActionResult::InProgress(ctrlr) => {
                    controller = ctrlr;
                }
            }
        }

        return ActionResult::InProgress(controller);
    }

    fn render(&self) {}

    fn interruptible(&self) -> bool {
        false
    }

    fn kickoff(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        String::from("BasicKickoffAction")
    }
}
