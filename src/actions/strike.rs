use rlbot_lib::rlbot::{
    ControllerState, GameTickPacket, Physics, PredictionSlice, RenderMessage, Vector3,
};

use crate::utils::{intercept::Intercept, math::math::Vec3, ActionTickResult};

use super::{
    action::{Action, ActionResult},
    goto_action::GotoAction,
};

pub struct StrikeAction {
    pub target: Option<Vector3>,
    pub goto: Option<GotoAction>,
    pub intercept: Option<Intercept>,
    pub last_update_time: f32,
    pub finished: bool,
    update_interval: f32,
    stop_updating: f32,
    max_additional_time: f32,
    car_id: usize,
    initial_time: f32,
    configure: Option<Box<dyn Fn(Vector3, &mut GotoAction, &Intercept)>>,
    use_intercept_predicate: bool,
}

impl StrikeAction {
    pub fn new(
        car_id: usize,
        target: Option<Vector3>,
        configure: Option<Box<dyn Fn(Vector3, &mut GotoAction, &Intercept)>>,
        use_intercept_predicate: bool,
    ) -> StrikeAction {
        StrikeAction {
            update_interval: 0.2,
            stop_updating: 0.1,
            max_additional_time: 0.4,
            car_id,
            target,
            goto: None,
            intercept: None,
            last_update_time: 0.,
            initial_time: -1.,
            finished: true,
            configure,
            use_intercept_predicate,
        }
    }

    pub fn best_target(
        car_phys: Physics,
        ball: Physics,
        their_goal: &Vector3,
        targets: &Vec<Vector3>,
    ) -> Option<Vector3> {
        let car_loc = &car_phys.location.unwrap().ground();
        let ball_loc = ball.location.unwrap().ground();
        let to_goal = their_goal.ground().sub(&car_loc);
        targets
            .iter()
            .max_by(|a, b| {
                car_loc
                    .sub(&ball_loc)
                    .add(&to_goal.scale(0.5))
                    .dot(&ball_loc.sub(&a))
                    .partial_cmp(
                        &car_loc
                            .sub(&ball_loc)
                            .add(&to_goal.scale(0.5))
                            .dot(&ball_loc.sub(&b)),
                    )
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    /// setup the goto action based on our intercept
    pub fn default_configure(&mut self, intercept: Intercept) {
        match self.goto.as_mut() {
            Some(goto) => {
                goto.target = intercept.location.clone();
                goto.arrival_time = intercept.time;
            }
            None => {
                self.goto = Some(GotoAction::new(intercept.location.clone(), None, self.car_id));
            }
        }

        self.intercept = Some(intercept);
    }

    pub fn update_intercept(
        &mut self,
        tick_packet: &GameTickPacket,
        predictions: &Vec<PredictionSlice>,
    ) {
        let game_time = tick_packet.gameInfo.clone().unwrap().secondsElapsed;
        if self.initial_time == -1. {
            self.initial_time = game_time;
        }

        let players = tick_packet.clone().players.unwrap();
        let car = players.get(self.car_id).unwrap();
        let car_phys = car.physics.clone().unwrap();
        let _car_location = car_phys.location.clone().unwrap();
        let _rotation = car_phys.rotation.clone().unwrap();
        let _velocity = car_phys.velocity.clone().unwrap();
        let ball = tick_packet.ball.clone().unwrap().physics.unwrap();
        let intercept = Intercept::new(&car, game_time, &predictions, *ball, false, self.use_intercept_predicate);

        if let Some(conf) = self.configure.as_ref() {
            let mut goto = self.goto.clone();
            conf(self.target.clone().unwrap(), goto.as_mut().unwrap(), &intercept);
            self.goto = goto;
        } else {
            self.default_configure(intercept);
        }
        self.last_update_time = game_time;
        if let Some(incpt) = self.intercept.as_ref() {
            if !incpt.is_viable || incpt.time > self.initial_time + self.max_additional_time {
                self.finished = true;
            }
        }
    }
}

impl Action for StrikeAction {
    fn step(
        &mut self,
        tick_packet: GameTickPacket,
        controller: ControllerState,
        predictions: &Vec<PredictionSlice>,
        dt: f32,
    ) -> ActionResult {
        if self.intercept.is_none() {
            return ActionResult::Failed;
        }
        let mut action_result = ActionTickResult::from(controller);
        let incpt = self.intercept.clone().unwrap();
        let game_time = tick_packet.gameInfo.clone().unwrap().secondsElapsed;
        let players = tick_packet.clone().players.unwrap();
        let car = players.get(self.car_id).unwrap();
        let car_phys = car.physics.clone().unwrap();
        let _car_location = car_phys.location.clone().unwrap();
        let _rotation = car_phys.rotation.clone().unwrap();
        let _velocity = car_phys.velocity.clone().unwrap();
        let _ball = tick_packet.ball.clone().unwrap().physics.unwrap();

        if self.last_update_time + self.update_interval < game_time
            && game_time < incpt.time - self.stop_updating
            && car.hasWheelContact
            && !action_result.controller.jump
        {
            println!("update_intercept");
            self.update_intercept(&tick_packet, predictions);
        }
        if incpt.time - game_time > 1. && self.interruptible() && !car.hasWheelContact {
            self.finished = true;
        }
        if let Some(goto) = self.goto.as_mut() {
            match goto.step(tick_packet, action_result.controller.clone(), predictions, dt) {
                ActionResult::InProgress(res) => {
                    action_result.controller = res.controller;
                    if goto.drive.target_speed < 300. {
                        action_result.controller.throttle = 0.;
                    };
                }
                _ => {
                    self.finished = true;
                }
            }
        }
        if self.finished {
            ActionResult::Success
        } else {
            ActionResult::InProgress(action_result)
        }
    }

    fn render(&self) -> Vec<RenderMessage> {
        vec![]
    }

    fn interruptible(&self) -> bool {
        if let Some(goto) = self.goto.as_ref() {
            goto.interruptible()
        } else {
            true
        }
    }

    fn kickoff(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        String::from("StrikeAction")
    }
}
