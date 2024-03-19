/// this is where the bot goes. There will be a struct here called Bot, and some implemented
/// methods that handle all the logic and stuff.

/// This file should *not* contain any code that deals with the connection

pub mod bot {
    use rlbot_lib::rlbot::{
        ControllerState, GameTickPacket, Physics, PlayerConfiguration, PlayerInput,
    };
    use crate::{actions::action::Action, solo_strategy::strategy::SoloStrategy};

    pub struct Agent {
        player_config: PlayerConfiguration,
        debug_rendering: bool,
        /// The last known car Physics Object
        phys: Box<Physics>,
        car_id: usize,
        player_id: i32,
        last_touch_time: Option<f32>,
        /// Used to ignore the first 20 ticks (which aren't useful or something)
        tick_count: i32,
        /// Whatever the bot is currently trying to do
        current_action: Option<Box<dyn Action>>,
        strategy: SoloStrategy,
    }

    impl Agent {
        fn handle_game_tick(mut self, packet: GameTickPacket) -> PlayerInput {
            let target = packet.ball.unwrap().physics.unwrap();
            let tmp = packet.players.unwrap();
            let car = tmp.get(self.car_id).expect("There is no game.");
            let boost = car.boost;
            self.phys = car.physics.clone().unwrap();

            let bot_to_target_angle = (target.location.clone().unwrap().y
                - self.phys.location.clone().unwrap().y)
                .atan2(target.location.unwrap().x - self.phys.location.unwrap().x);

            let mut bot_front_to_target_angle =
                bot_to_target_angle - self.phys.rotation.unwrap().yaw;

            if bot_front_to_target_angle > 3.14 {
                bot_front_to_target_angle -= 2. * 3.14
            }
            if bot_front_to_target_angle < -3.14 {
                bot_front_to_target_angle += 2. * 3.14
            }

            let mut controller = ControllerState::default();

            if bot_front_to_target_angle > 0. {
                controller.steer = 1.;
            } else {
                controller.steer = -1.;
            }

            controller.throttle = 1.;

            // NOTE: with this tick rate, we end on 14 boost
            if boost > 15 {
                controller.boost = true;
            }

            // connection
            //     .send_packet(Packet::PlayerInput(PlayerInput {
            //         playerIndex: car_id as i32,
            //         controllerState: Some(Box::new(controller)),
            //     }))
            //     .unwrap();
            //
            // connection

            // ==== Start of the Botimus prime stuff ====

            if self.tick_count < 20 {
                self.tick_count += 1;
                return PlayerInput { playerIndex: self.player_id, controllerState: None };
            };

            // self.info.read_packet(packet)
            let target = packet.ball.unwrap().physics.unwrap();
            let tmp = packet.players.unwrap();
            let car = tmp.get(self.car_id).expect("There is no game.");
            let boost = car.boost;
            self.phys = car.physics.clone().unwrap();

            // if packet.game_info.is_kickoff_pause and not isinstance(self.maneuver, Kickoff):
            //     self.maneuver = None

            // cancel current_action if a kickoff is happening and current_action isn't a kickoff
            if let Some(action) = self.current_action {
                if packet.gameInfo.unwrap().isKickoffPause && !action.kickoff() {
                    self.current_action = None;
                }
            }

            // reset maneuver when another car hits the ball
            let touch = packet.ball.unwrap().latestTouch.unwrap();
            if let Some(last_touch) = self.last_touch_time {
                if touch.gameSeconds > last_touch && touch.playerName != packet.players.unwrap()[self.car_id].name {
                    self.last_touch_time = Some(touch.gameSeconds);

                    // don't reset when we're dodging, wavedashing or recovering
                    if let Some(action) = self.current_action {
                        if action.interruptible() {
                            self.current_action = None;
                        }
                    }
                }
            }

            // choose maneuver
            if self.current_action.is_none() {
                // TODO: implement debug rendering
                // if self.debug_rendering {
                // }

                // We're assuming that we have an appropriate strategy passed on startup.
            }
            // if self.maneuver is None:
            //     if self.RENDERING:
            //         self.draw.clear()
            //
            //     if self.info.get_teammates(self.info.cars[self.index]):
            //         self.maneuver = teamplay_strategy.choose_maneuver(self.info, self.info.cars[self.index])
            //     else:
            //         self.maneuver = solo_strategy.choose_maneuver(self.info, self.info.cars[self.index])
            //
            // # execute maneuver
            // if self.maneuver is not None:
            //     self.maneuver.step(self.info.time_delta)
            //     self.controls = self.maneuver.controls
            //
            //     if self.RENDERING:
            //         self.draw.group("maneuver")
            //         self.draw.color(self.draw.yellow)
            //         self.draw.string(self.info.cars[self.index].position + vec3(0, 0, 50), type(self.maneuver).__name__)
            //         self.maneuver.render(self.draw)
            //
            //     # cancel maneuver when finished
            //     if self.maneuver.finished:
            //         self.maneuver = None
            //
            // if self.RENDERING:
            //     self.draw.execute()

            PlayerInput { playerIndex: self.player_id, controllerState: None }
        }
    }
}
