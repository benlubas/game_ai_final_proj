/// this is where the bot goes. There will be a struct here called Bot, and some implemented
/// methods that handle all the logic and stuff.

/// This file should *not* contain any code that deals with the connection

pub mod bot {
    use rlbot_lib::rlbot::{ControllerState, GameTickPacket, Physics, PlayerInput};

    use crate::actions::action::{Action, ActionResult};
    use crate::solo_strategy::strategy::SoloStrategy;

    pub struct Agent {
        // pub player_config: PlayerConfiguration, // I'm not sure what this is used for
        pub debug_rendering: bool,
        /// The last known car Physics Object
        phys: Option<Box<Physics>>,
        pub car_id: usize,
        last_touch_time: Option<f32>,
        /// Used to ignore the first 20 ticks (which aren't useful or something)
        tick_count: i32,
        /// the last secondsElapsed value we saw
        last_tick_time: f32,
        /// Whatever the bot is currently trying to do
        current_action: Option<Box<dyn Action>>,
        pub strategy: SoloStrategy,
        current_controller: ControllerState,
    }

    impl Agent {
        pub fn new(debug: bool, car_id: usize, strategy: SoloStrategy) -> Agent {
            Agent {
                debug_rendering: debug,
                phys: None,
                car_id,
                last_touch_time: None,
                tick_count: 0,
                last_tick_time: 0.,
                current_action: None,
                strategy,
                current_controller: ControllerState::default(),
            }
        }

        pub fn handle_game_tick(&mut self, packet: GameTickPacket) -> PlayerInput {
            // Ignore the first 20 ticks
            if self.tick_count < 20 {
                self.tick_count += 1;
                return PlayerInput {
                    playerIndex: self.car_id as i32,
                    controllerState: Some(Box::new(ControllerState::default())),
                };
            };

            let ball = packet.clone().ball.unwrap();
            let _target = ball.clone().physics.unwrap();
            let tmp = packet.clone().players.unwrap();
            let car = tmp.get(self.car_id).expect("There is no game.");
            let _boost = car.boost;
            self.phys = car.physics.clone();
            let _car_phys = car.physics.clone().unwrap();
            let seconds_elapsed = packet.gameInfo.clone().unwrap().secondsElapsed;
            let dt = seconds_elapsed - self.last_tick_time;
            self.last_tick_time = seconds_elapsed;
            let is_kickoff = packet.gameInfo.clone().unwrap().isKickoffPause;
            // cancel current_action if a kickoff is happening and current_action isn't a kickoff
            if let Some(action) = &self.current_action {
                if is_kickoff && !action.kickoff() {
                    println!("Clearing for kickoff");
                    self.current_action = None;
                }
            }

            // reset maneuver when another car hits the ball
            if let Some(touch) = ball.latestTouch.clone() {
                if let Some(last_touch) = self.last_touch_time {
                    if touch.gameSeconds > last_touch
                        && touch.playerName.unwrap_or(String::from(""))
                            != packet.players.clone().unwrap()[self.car_id]
                                .name
                                .clone()
                                .unwrap_or(String::from(" "))
                    {
                        self.last_touch_time = Some(touch.gameSeconds);

                        // don't reset when we're dodging, wavedashing or recovering
                        if let Some(action) = &self.current_action {
                            if action.interruptible() {
                                self.current_action = None;
                            }
                        }
                    }
                }
            }

            // choose maneuver
            if self.current_action.is_none() {
                // TODO: implement debug rendering
                // if self.debug_rendering {
                // }

                println!("Assigning new Action");
                self.current_action = self.strategy.choose_action(packet.clone(), is_kickoff);
                if let Some(action) = &self.current_action {
                    println!("Choosen Action: {}", action.name());
                }
            }

            let mut controller = self.current_controller.clone();

            if let Some(action) = self.current_action.as_mut() {
                if let ActionResult::InProgress(ctrlr) = action.step(packet.clone(), controller.clone(), dt) {
                    controller = ctrlr;
                } else {
                    self.current_action = None;
                }
            }

            // if self.debug_rendering {
            //     // TODO: draw shit?
            // }

            self.current_controller = controller;
            PlayerInput {
                playerIndex: self.car_id as i32,
                controllerState: Some(Box::new(self.current_controller.clone())),
            }
        }
    }
}
