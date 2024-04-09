use rlbot_lib::rlbot::{GameTickPacket, DesiredGameState};

use crate::actions::{
    action::Action, drive_action::DriveAction, kickoff_action::BasicKickoffAction,
};

use super::strategy::Strategy;

pub struct SoloStrategy {}

impl Strategy for SoloStrategy {
    // TODO: GameTickPacket has the ball info in it... I should write a function that parses
    // that into a struct... or something.
    fn choose_action(
        &self,
        tick_packet: GameTickPacket,
        kickoff: bool,
    ) -> Option<Box<dyn Action>> {
        let ball = tick_packet.ball.unwrap();

        // For now, we're just always driving at the ball.
        let ball_location = ball.physics.unwrap().location.unwrap();

        let action: Box<dyn Action>;
        if kickoff {
            action = Box::new(BasicKickoffAction::new());
        } else {
            action = Box::new(DriveAction {
                target_pos: ball_location,
                target_speed: 1300.0,
                drive_on_walls: false,
            });
        }

        Some(action)
    }

    fn set_game_state(&self) -> Option<DesiredGameState> {
        None
    }
}
