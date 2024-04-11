use rlbot_lib::rlbot::{DesiredGameState, GameTickPacket, PlayerInfo, PredictionSlice};

use crate::{
    actions::{action::Action, drive_action::DriveAction, kickoff_action::BasicKickoffAction},
    utils::{
        arena::Arena,
        boost::pads::{choose_boostpad, BoostPad},
        intercept::Intercept,
        math::math::Vec3,
    },
    DEFAULT_CAR_ID,
};

use super::strategy::Strategy;

pub struct SoloStrategy {}

impl Strategy for SoloStrategy {
    // TODO: GameTickPacket has the ball info in it... I should write a function that parses
    // that into a struct... or something.
    fn choose_action(
        &self,
        tick_packet: GameTickPacket,
        ball_predictions: &Vec<PredictionSlice>,
        kickoff: bool,
    ) -> Option<Box<dyn Action>> {
        let ball = tick_packet.clone().ball.unwrap();
        let ball_phys = ball.physics.clone().unwrap();
        let game_time = tick_packet.clone().gameInfo.unwrap().secondsElapsed;
        let players = tick_packet.players.clone().unwrap();
        let car = players.get(DEFAULT_CAR_ID).unwrap();
        let car_phys = car.physics.clone().unwrap();
        let car_location = car_phys.location.clone().unwrap();
        let mut opponents = players.clone();
        opponents.remove(DEFAULT_CAR_ID);

        let my_goal = Arena::home_goal_pos(car.team);
        let their_goal = Arena::enemy_goal_pos(car.team);

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

        let my_intercept = Intercept::new(car, game_time, ball_predictions, *ball_phys.clone(), false);
        let their_intercept = opponents
            .into_iter()
            .map(|opp_car| Intercept::new(&opp_car, game_time, ball_predictions, *ball_phys.clone(), false))
            .min_by(|a, b| {
                a.time
                    .partial_cmp(&b.time)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        // we might have no opponent if they're demoed or leave the game, don't want to crash
        let mut opponent: Option<PlayerInfo> = None;
        if let Some(op) = their_intercept {
            opponent = Some(op.car);
        }

        let pads = BoostPad::extract_info(&tick_packet.clone());
        let bad_pads: Vec<BoostPad> = pads
            .into_iter()
            .filter(|pad| {
                ((pad.location.y - their_goal.y).abs()
                    < (my_intercept.location.y - their_goal.y).abs()
                    || (pad.location.x - car_location.x).abs() > 6000.)
                    && car_location.ground().dist(&pad.location.ground()) < 4000.
            })
            .collect();
        let best_boost = choose_boostpad(&tick_packet.clone(), car.clone(), &my_goal, bad_pads);

        // # if ball is in a dangerous position, clear it
        // if (
        //         ground_distance(my_intercept, my_goal) < 3000
        //         and (abs(my_intercept.position[0]) < 2000 or abs(my_intercept.position[1]) < 4500)
        //         and my_car.position[2] < 300
        // ):
        if my_intercept.location.ground().dist(&my_goal.ground()) < 3000.
            && my_intercept.location.x.abs() < 2000.
            || my_intercept.location.y.abs() < 4500. && car_location.z < 300.
        {
            todo!("Shoot the ball/clear the ball");
        }
        //     if align(my_car.position, my_intercept.ball, their_goal) > 0.5:
        //         return offense.any_shot(info, my_intercept.car, their_goal, my_intercept, allow_dribble=True)
        //     return defense.any_clear(info, my_intercept.car)
        //
        // # if I'm low on boost and the ball is not near my goal, go for boost
        // if my_car.boost < 10 and ground_distance(my_intercept, their_goal) > 3000 and best_boostpad_to_pickup is not None:
        //     return PickupBoostPad(my_car, best_boostpad_to_pickup)

        Some(action)
    }

    fn set_game_state(&self) -> Option<DesiredGameState> {
        None
    }
}
