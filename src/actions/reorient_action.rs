use rlbot_lib::rlbot::{ControllerState, GameTickPacket, PredictionSlice, RenderMessage, Vector3};

use crate::utils::{
    math::math::{vec_new, Rot3, Vec3},
    ActionTickResult,
};

use super::action::{Action, ActionResult};

pub struct ReorientAction {
    pub car_id: usize,
    target_up: Vector3,
    target_forward: Vector3,
}

impl ReorientAction {
    pub fn from_uf(up: Vector3, forward: Vector3, car_id: usize) -> ReorientAction {
        ReorientAction {
            target_up: up,
            target_forward: forward,
            car_id,
        }
    }
}

// NOTE: This is kinda awful, I think the coordinate system is messing with me again, but I don't
// have time to figure it out
impl Action for ReorientAction {
    fn step(
        &mut self,
        tick_packet: GameTickPacket,
        controller: ControllerState,
        predictions: &Vec<PredictionSlice>,
        _dt: f32,
    ) -> ActionResult {
        let controller = controller.clone();
        let mut action_result = ActionTickResult::from(controller);
        let players = tick_packet.clone().players.clone().unwrap();
        let car = players.get(self.car_id).clone().unwrap();
        let car_phys = car.physics.clone().unwrap();
        let car_rotation = car_phys.rotation.clone().unwrap();

        let target_rot = nalgebra::Rotation3::face_towards(
            &vec_new(
                self.target_forward.x,
                -self.target_forward.y,
                self.target_forward.z,
            )
            .to_nalg(),
            &self.target_up.to_nalg(),
        );
        let rot_to = target_rot.rotation_to(&car_rotation.to_nalg());
        let needed_rotations = rot_to.euler_angles();

        action_result.controller.roll = needed_rotations.0;
        action_result.controller.pitch = needed_rotations.1;
        action_result.controller.yaw = needed_rotations.2;

        if action_result.controller.roll.abs() < 1e-4
            && action_result.controller.pitch.abs() < 1e-4
            && action_result.controller.yaw.abs() < 1e-4
        {
            ActionResult::Success
        } else if car.hasWheelContact {
            ActionResult::Failed
        } else {
            ActionResult::InProgress(action_result)
        }
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
        String::from("ReorientAction")
    }
}
