// class Jump:
//     def __init__(self, duration):
//
//         self.duration = duration
//         self.controls = Input()
//
//         self.timer = 0
//         self.counter = 0
//
//         self.finished = False
//
//     def interruptible(self) -> bool:
//         return False
//
//     def step(self, dt):
//
//         self.controls.jump = 1 if self.timer < self.duration else 0
//
//         if self.controls.jump == 0:
//             self.counter += 1
//
//         self.timer += dt
//
//         if self.counter >= 2:
//             self.finished = True
//
//         return self.finished

use rlbot_lib::rlbot::{GameTickPacket, ControllerState};

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
    fn step(&mut self, tick_packet: GameTickPacket, controller: ControllerState, dt: f32) -> super::action::ActionResult {
        let jump = self.timer < self.duration;
        if !jump {
            self.counter += 1;
        }
        self.timer += dt;

        if self.counter >= 2 {
            return ActionResult::Success
        } else {
            return ActionResult::InProgress(ControllerState {
                jump,
                ..controller
            })
        }
    }

    fn render(&self) {
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
