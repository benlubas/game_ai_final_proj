// Actions

mod kickoff_action;
mod drive_action;

pub mod action {
    use rlbot_lib::rlbot::ControllerState;


    pub enum ActionResult {
        Success,
        Failed,
        InProgress(ControllerState),
    }

    pub trait Action {
        fn step(&self, dt: f32) -> ActionResult;
        fn render(&self);
        fn interruptible(&self) -> bool;
        fn kickoff(&self) -> bool;
    }
}
