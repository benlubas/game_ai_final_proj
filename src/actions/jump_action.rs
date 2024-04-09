use rlbot_lib::rlbot::{GameTickPacket, ControllerState, RenderMessage};

use crate::utils::ActionTickResult;

use super::action::{Action, ActionResult};

pub struct JumpAction {
    pub duration: f32,
    timer: f32,
    counter: i32,
}

impl JumpAction {
    pub fn new(duration: f32) -> JumpAction {
        JumpAction {
            duration,
            timer: 0.,
            counter: 0,
        }
    }
}

impl Action for JumpAction {
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
        String::from("JumpAction")
    }
}
