pub mod solo_strategy;
pub mod test_strategy;

pub mod strategy {
    use rlbot_lib::rlbot::{GameTickPacket, DesiredGameState, PredictionSlice};

    use crate::actions::action::Action;

    pub trait Strategy {
        fn choose_action(
            &self,
            tick_packet: GameTickPacket,
            ball_predictions: &Vec<PredictionSlice>,
            kickoff: bool,
        ) -> Option<Box<dyn Action>>;

        fn set_game_state(&self) -> Option<DesiredGameState>;
    }
}
