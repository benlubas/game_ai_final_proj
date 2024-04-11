use std::f32::consts::PI;

use rlbot_lib::rlbot::{
    DesiredCarState, DesiredGameState, DesiredPhysics, Float, GameTickPacket, RotatorPartial, PredictionSlice,
};

use rlbot_lib::rlbot::Vector3Partial;

use crate::{
    actions::{action::Action, recover_action::RecoverAction},
    utils::arena::Arena,
};
use rand::Rng;

use super::strategy::Strategy;

pub struct TestStrategy {}

impl Strategy for TestStrategy {
    fn choose_action(
        &self,
        _tick_packet: GameTickPacket,
        _ball_predictions: &Vec<PredictionSlice>,
        _kickoff: bool,
    ) -> Option<Box<dyn Action>> {
        // let on_ground = tick_packet.players.unwrap().get(DEFAULT_CAR_ID).unwrap().hasWheelContact;
        Some(Box::new(RecoverAction::new(false)))
    }

    fn set_game_state(&self) -> Option<DesiredGameState> {
        let mut rng = rand::thread_rng();
        // Pick a random position, rotation, and velocity to give to the car
        let position = Arena::random_pos(300.);
        let position = Vector3Partial {
            x: Some(Float { val: position.x }),
            y: Some(Float { val: position.y }),
            z: Some(Float { val: position.z }),
        };
        let velocity = Vector3Partial {
            x: Some(Float {
                val: rng.gen_range(-1200.0..1200.),
            }),
            y: Some(Float {
                val: rng.gen_range(-1200.0..1200.),
            }),
            z: Some(Float {
                val: rng.gen_range(0.0..1200.),
            }),
        };
        let rotation = RotatorPartial {
            pitch: Some(Float {
                val: rng.gen_range((-PI / 2.)..(PI / 2.)),
            }),
            yaw: Some(Float {
                val: rng.gen_range(-PI..PI),
            }),
            roll: Some(Float {
                val: rng.gen_range(-PI..PI),
            }),
        };

        let car_state = DesiredCarState {
            physics: Some(Box::new(DesiredPhysics {
                location: Some(Box::new(position)),
                rotation: Some(Box::new(rotation)),
                velocity: Some(Box::new(velocity)),
                ..Default::default()
            })),
            boostAmount: Some(Float {
                val: rng.gen_range(0.0..100.),
            }),
            jumped: None,
            doubleJumped: None,
        };

        println!("Car state: {car_state:?}");

        Some(DesiredGameState {
            carStates: Some(vec![car_state]),
            ..Default::default()
        })
    }
}
