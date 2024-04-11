use rlbot_lib::rlbot::{GameTickPacket, ControllerState, RenderMessage};

use crate::utils::ActionTickResult;

use super::action::{Action, ActionResult};

pub struct DriveShotAction {
    pub duration: f32,
    timer: f32,
    counter: i32,
}

impl DriveShotAction {
    pub fn new(duration: f32) -> DriveShotAction {
        DriveShotAction {
            duration,
            timer: 0.,
            counter: 0,
        }
    }
}
 // max_distance_from_wall = 120
 //    max_additional_time = 0.3
 //
 //    def intercept_predicate(self, car: Car, ball: Ball):
 //        if ball.position[2] > 200 or abs(ball.position[1]) > Arena.size[1] - 100:
 //            return False
 //        contact_ray = Field.collide(sphere(ball.position, self.max_distance_from_wall))
 //        return norm(contact_ray.start) > 0 and abs(dot(ball.velocity, contact_ray.direction)) < 300
 //
 //    def configure(self, intercept: Intercept):
 //        target_direction = ground_direction(intercept, self.target)
 //        strike_direction = ground_direction(intercept.ball.velocity, target_direction * 4000)
 //        
 //        self.arrive.target = intercept.position - strike_direction * 105
 //        self.arrive.target_direction = strike_direction
 //        self.arrive.arrival_time = intercept.time

impl Action for DriveShotAction {
    fn step(&mut self, _tick_packet: GameTickPacket, controller: ControllerState, dt: f32) -> super::action::ActionResult {
        let jump = self.timer < self.duration;
        if !jump {
            self.counter += 1;
        }
        self.timer += dt;

        if self.counter >= 2 {
            return ActionResult::Success
        } else {
            return ActionResult::InProgress(ActionTickResult::from(ControllerState {
                jump,
                ..controller
            }))
        }
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
        String::from("DriveShotAction")
    }
}
