use rlbot_lib::rlbot::{
    ControllerState, GameTickPacket, PredictionSlice, RenderMessage, Vector3,
};

use crate::utils::{intercept::Intercept, math::math::Vec3};

use super::{
    action::{Action, ActionResult},
    strike::StrikeAction, goto_action::GotoAction,
};

pub struct DriveShotAction {
    strike: StrikeAction,
}

impl DriveShotAction {
    pub fn new(car_id: usize, target: Vector3) -> DriveShotAction {
        DriveShotAction {
            strike: StrikeAction::new(
                car_id,
                Some(target),
                Some(Box::new(|target: Vector3, goto: &mut GotoAction, intercept: &Intercept| {
                    let target_direction = intercept.location.ground().sub(&target);
                    let strike_direction = intercept
                        .ball
                        .velocity
                        .clone()
                        .unwrap()
                        .ground()
                        .sub(&target_direction.scale(4000.));
                    goto.target = intercept.location.sub(&strike_direction.scale(105.));
                    goto.target_direction = Some(strike_direction);
                    goto.arrival_time = intercept.time;
                })),
                true,
            ),
        }
    }
}

impl Action for DriveShotAction {
    fn step(
        &mut self,
        tick_packet: GameTickPacket,
        controller: ControllerState,
        predictions: &Vec<PredictionSlice>,
        dt: f32,
    ) -> ActionResult {
        self.strike.step(tick_packet, controller, predictions, dt)
    }

    fn render(&self) -> Vec<RenderMessage> {
        vec![]
    }

    fn interruptible(&self) -> bool {
        false
    }

    fn kickoff(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        if let Some(incpt) = self.strike.intercept.as_ref() {
            let time = incpt.time.to_string();
            return format!("DriveShotAction ({time})")
        }
        format!("DriveShotAction")
    }
}
