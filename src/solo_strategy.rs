pub mod strategy {

    use rlbot_lib::rlbot::{BallInfo, GameTickPacket};

    use crate::actions::{
        action::Action, drive_action::DriveAction, kickoff_action::BasicKickoffAction,
    };

    pub struct SoloStrategy {}

    impl SoloStrategy {
        // TODO: GameTickPacket has the ball info in it... I should write a function that parses
        // that into a struct... or something.
        pub fn choose_action(
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

            // ball = info.ball
            // their_goal = ground(info.their_goal.center)
            // my_goal = ground(info.my_goal.center)
            // opponents = info.get_opponents()
            //
            // # recovery
            // if not my_car.on_ground:
            //     return Recovery(my_car)
            //
            // # kickoff
            // if ball.position[0] == 0 and ball.position[1] == 0:
            //     return kickoffs.choose_kickoff(info, my_car)
            //
            // info.predict_ball()
        }
    }
}
