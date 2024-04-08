// Actions

pub mod kickoff_action;
pub mod drive_action;
pub mod jump_action;
pub mod airdodge_action;

pub mod action {
    use rlbot_lib::rlbot::{ControllerState, GameTickPacket};

    use super::{drive_action::DriveAction, kickoff_action::BasicKickoffAction};


    pub enum ActionResult {
        Success,
        Failed,
        InProgress(ControllerState),
    }

    pub enum Actions {
        Drive(DriveAction),
        BasicKickoff(BasicKickoffAction),
    }

    pub trait Action {
        fn step(&mut self, tick_packet: GameTickPacket, controller: ControllerState, dt: f32) -> ActionResult;
        fn render(&self);
        fn interruptible(&self) -> bool;
        fn kickoff(&self) -> bool;
        fn name(&self) -> String;
    }
}
