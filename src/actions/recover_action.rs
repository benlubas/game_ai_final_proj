use rlbot_lib::rlbot::{ControllerState, GameTickPacket, Vector3, RenderMessage, Physics, Rotator};

use crate::{utils::{
    arena::Arena,
    math::math::{abs_clamp, forward_vec, up_vec, Vec3}, ActionTickResult,
}, DEFAULT_CAR_ID};

use super::action::{Action, ActionResult};

pub struct RecoverAction {
    // track the progress of this action, b/c this is a timed uninterruptible action
    pub jump_when_upside_down: bool,
    landing_pos: Option<Vector3>,
    trajectory: Vec<Vector3>,
    landing: bool,
}

impl RecoverAction {
    pub fn new(jump_when_upside_down: bool) -> RecoverAction {
        RecoverAction {
            jump_when_upside_down,
            landing_pos: None,
            trajectory: vec![],
            landing: false,
        }
    }

    pub fn simulate_landing(&mut self, car_phys: Physics) {
        let mut car_pos = car_phys.location.unwrap();
        let mut car_vel = car_phys.velocity.unwrap();
        let gravity = Vector3 { x: 0., y: 0., z: -650. };

        self.trajectory = vec![car_pos];
        self.landing = false;
        let mut collision_normal: Option<Vector3> = None;

        let dt = 1. / 60.;
        let simulation_duration: f32 = 0.8;
        for i in 0..((simulation_duration / dt) as i32) {
            car_pos.add(&car_vel.scale(dt));
            car_vel.add(&gravity.scale(dt));

            if car_vel.norm() > 2300. {
                car_vel = car_vel.normalize().scale(2300.);
            }
            self.trajectory.push(car_pos);
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
            let l = u.cross(&f).normalize();
        }
        // if self.landing:
        //     u = collision_normal
        //     f = normalize(vel - dot(vel, u) * u)
        //     l = normalize(cross(u, f))
        //     self.reorient.target_orientation = three_vec3_to_mat3(f, l, u)
        // else:
        //     target_direction = normalize(normalize(self.car.velocity) - vec3(0, 0, 3))
        //     self.reorient.target_orientation = look_at(target_direction, vec3(0, 0, 1))
    }

    // Okay, so trying to get the way to rotate things.
    //
    // I think the best bet is to match the up vector with the normal vector
    // and then determine the yaw based on the car's velocity (like project the velocity onto the
    // plane we land on and then match the forward vector to that by rotating in yaw).
    pub fn controller_from_target_orientation(f: Vector3, l: Vector3, u: Vector3) -> Rotator {
        Rotator {
            roll: 0.,
            pitch: 0.,
            yaw: 0.,
        }
    }
}

// This is blocked on basic drive and flip actions
impl Action for RecoverAction {
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

        // self.simulate_landing()
        // self.reorient.step(dt)
        // self.controls = self.reorient.controls
        //
        // self.controls.boost = angle_between(self.car.forward(), vec3(0, 0, -1)) < 1.5 and not self.landing
        // self.controls.throttle = 1  # in case we're turtling
        //
        // # jump if the car is upside down and has wheel contact
        // if (
        //     self.jump_when_upside_down
        //     and self.car.on_ground
        //     and dot(self.car.up(), vec3(0, 0, 1)) < -0.95
        // ):
        //     self.controls.jump = True
        //     self.landing = False
        //     
        // else:
        //     self.finished = self.car.on_ground
    }

    fn render(&self) -> Vec<RenderMessage> {
        vec![]
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
