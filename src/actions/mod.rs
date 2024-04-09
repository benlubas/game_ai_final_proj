// Actions

pub mod airdodge_action;
pub mod drive_action;
pub mod jump_action;
pub mod kickoff_action;
pub mod recover_action;

pub mod action {
    use rlbot_lib::rlbot::{ControllerState, GameTickPacket, RenderMessage};

    use crate::utils::ActionTickResult;

    pub enum ActionResult {
        Success,
        Failed,
        InProgress(ActionTickResult),
    }

    pub trait Action {
        fn step(
            &mut self,
            tick_packet: GameTickPacket,
            controller: ControllerState,
            dt: f32,
        ) -> ActionResult;
        fn render(&self) -> Vec<RenderMessage>;
        fn interruptible(&self) -> bool;
        fn kickoff(&self) -> bool;
        fn name(&self) -> String;
    }
}
