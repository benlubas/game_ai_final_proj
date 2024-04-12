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

        let my_intercept = Intercept::new(
            car,
            game_time,
            ball_predictions,
            *ball_phys.clone(),
            false,
            false,
        );

        let their_intercept = opponents
            .into_iter()
            .map(|opp_car| {
                Intercept::new(
                    &opp_car,
                    game_time,
                    ball_predictions,
                    *ball_phys.clone(),
                    false,
                    false,
                )
            })
            .min_by(|a, b| {
                a.time
                    .partial_cmp(&b.time)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        // we might have no opponent if they're demoed or leave the game, don't want to crash
        let mut _opponent: Option<PlayerInfo> = None;
        if let Some(op) = their_intercept {
            _opponent = Some(op.car);
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

        let mut action: Box<dyn Action>;
        if kickoff {
            return Some(Box::new(BasicKickoffAction::new()));
        } else if my_intercept.is_viable {
            // default drive to intercept the ball
            action = Box::new(DriveAction::new(
                my_intercept.location.clone(),
                2300.,
                false,
                false,
            ))
        } else {
            action = Box::new(DriveAction::new(ball_location.clone(), 2300., false, true))
        }

        // if ball is close to our net, clear it
        if my_intercept.location.ground().dist(&my_goal.ground()) < 3000.
            && my_intercept.location.x.abs() < 2000.
            || my_intercept.location.y.abs() < 4500. && car_location.z < 300.
        {
            // clear the ball (with a shot currently, this can be improved by determining the best
            // spot to clear based on the opponent location, I just ran out of time).
            if car_location.dist(&my_goal) > 2000. {
                // TODO: change this to goto with an angle towards the ball, maybe also goto
                // backpost or something.
                action = Box::new(DriveAction::new(my_goal.clone(), 2300., false, true));
            } else {
                // NOTE: drive shot action is very broken, needs a bunch of debugging/testing.
                // I wrote it in one shot without testing b/c i was running out of time.
                // action = Box::new(DriveShotAction::new(DEFAULT_CAR_ID, their_goal.clone()));
                action = Box::new(DriveAction::new(ball_location.clone(), 2300., false, false));
            }
        }
        // low and boost and ball isn't dangerous, so grab boost
        if let Some(boost_target) = best_boost {
            if car.boost < 30 && my_intercept.location.ground_dist(&their_goal) > 3000. {
                action = Box::new(DriveAction::new(boost_target.location, 2300., false, false))
            }
        }

        Some(action)
    }

    fn set_game_state(&self) -> Option<DesiredGameState> {
        None
    }
}
