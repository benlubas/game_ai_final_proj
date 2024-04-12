use rlbot_lib::rlbot::{ControllerState, GameTickPacket, Physics, RenderMessage, Vector3, PredictionSlice};

use crate::{
    utils::{
        arena::Arena,
        math::math::{up_vec, Vec3, vec_new, vec2_new},
        ActionTickResult, render::render::{cross, YELLOW, text},
    },
    DEFAULT_CAR_ID,
};

use super::{
    action::{Action, ActionResult},
    reorient_action::ReorientAction,
};

// NOTE: This is kinda awful, I think the coordinate system is messing with me again, but I don't
// have time to figure it out
pub struct RecoverAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    pub jump_when_upside_down: bool,
    landing_pos: Option<Vector3>,
    trajectory: Vec<Vector3>,
    landing: bool,
    reorient: Option<ReorientAction>,
}

impl RecoverAction {
    pub fn new(jump_when_upside_down: bool) -> RecoverAction {
        RecoverAction {
            jump_when_upside_down,
            landing_pos: None,
            trajectory: vec![],
            landing: false,
            reorient: None,
        }
    }

    pub fn simulate_landing(&mut self, car_phys: Physics) {
        let mut car_pos = car_phys.location.unwrap();
        let mut car_vel = car_phys.velocity.unwrap();
        let gravity = Vector3 {
            x: 0.,
            y: 0.,
            z: -650.,
        };

        self.trajectory = vec![car_pos.clone()];
        self.landing = false;
        let mut collision_normal: Option<Vector3> = None;

        let dt = 1. / 60.;
        let simulation_duration: f32 = 0.8;
        for i in 0..((simulation_duration / dt) as i32) {
            car_pos = car_pos.add(&car_vel.scale(dt));
            car_vel = car_vel.add(&gravity.scale(dt));

            if car_vel.norm() > 2300. {
                car_vel = car_vel.normalize().scale(2300.);
            }
            self.trajectory.push(car_pos.clone());
            let collission_result = Arena::collide(&car_pos);
            if collission_result.is_some() {
                collision_normal = collission_result;
                if i > 20 {
                    self.landing = true;
                    self.landing_pos = Some(car_pos);
                    break;
                }
            }
        }
        if self.landing {
            let u = collision_normal.unwrap();
            let f = car_vel.sub(&u.scale(car_vel.dot(&u))).normalize();
            self.reorient = Some(ReorientAction::from_uf(u, f, DEFAULT_CAR_ID));
        } else {
            let target_dir = car_vel
                .normalize()
                .sub(&Vector3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                })
                .normalize();
            self.reorient = Some(ReorientAction::from_uf(
                Vector3::up(),
                target_dir,
                DEFAULT_CAR_ID,
            ));
        }
    }
}

// This is blocked on basic drive and flip actions
impl Action for RecoverAction {
    fn step(
        &mut self,
        tick_packet: GameTickPacket,
        controller: ControllerState,
 predictions: &Vec<PredictionSlice>,
        dt: f32,
    ) -> ActionResult {
        let mut action_result = ActionTickResult::from(controller.clone());
        let players = tick_packet.clone().players.unwrap();
        let car = players.get(DEFAULT_CAR_ID).unwrap();
        let car_phys = car.clone().physics.clone().unwrap();
        // let car_location = car_phys.location.clone().unwrap();
        let rotation = car_phys.rotation.clone().unwrap();
        // let velocity = car_phys.velocity.clone().unwrap();

        self.simulate_landing(*car_phys.clone());
        if let Some(reorient) = self.reorient.as_mut() {
            match reorient.step(tick_packet.clone(), controller.clone(), predictions, dt) {
                ActionResult::InProgress(res) => {
                    action_result.controller = res.controller;
                }
                _ => {
                    action_result.controller.roll = 0.;
                    action_result.controller.pitch = 0.;
                    action_result.controller.yaw = 0.;
                }
            }
        }

        // self.controls.boost = angle_between(self.car.forward(), vec3(0, 0, -1)) < 1.5 and not self.landing

        action_result.controller.throttle = 1.; // in case we're turtling

        // # jump if the car is upside down and has wheel contact
        if self.jump_when_upside_down
            && car.hasWheelContact
            && up_vec(&rotation).dot(&Vector3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }) < -0.95
        {
            action_result.controller.jump = true;
            self.landing = false;
        } else if car.hasWheelContact {
            return ActionResult::Success;
        }

        ActionResult::InProgress(action_result)
    }

    fn render(&self) -> Vec<RenderMessage> {
        let mut renders = if let Some(target) = self.landing_pos.as_ref() {
            cross(&vec_new(target.x, target.y, target.z + 50.), 100., YELLOW)
        } else {
            vec![]
        };

        let more_text = if self.landing { "Landing" } else { "Pointing Down" };
        let mut t = vec![text(&vec2_new(20., 50.), String::from(more_text), YELLOW)];

        renders.append(&mut t);
        renders
    }

    fn interruptible(&self) -> bool {
        false
    }

    fn kickoff(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        String::from("RecoverAction")
    }
}
