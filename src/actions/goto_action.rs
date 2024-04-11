use rlbot_lib::rlbot::Vector3;

use crate::utils::{
    intercept::turn_radius,
    math::math::{clamp, forward_vec, Vec3},
};

use super::{
    action::{Action, ActionResult},
    drive_action::DriveAction,
};

pub struct GotoAction {
    drive: DriveAction,
    target: Vector3,
    target_direction: Option<Vector3>,
    car_id: usize,
    arrival_time: f32,
    additional_shift: f32,
    lerp_t: f32,
}

impl GotoAction {
    pub fn new(target: Vector3, target_direction: Option<Vector3>, car_id: usize) -> GotoAction {
        GotoAction {
            drive: DriveAction::new(target.clone(), 0., false),
            target,
            target_direction,
            car_id,
            arrival_time: f32::MAX,
            additional_shift: 0.,
            lerp_t: 0.56,
        }
    }
}

impl Action for GotoAction {
    fn step(
        &mut self,
        tick_packet: rlbot_lib::rlbot::GameTickPacket,
        controller: rlbot_lib::rlbot::ControllerState,
        dt: f32,
    ) -> ActionResult {
        let car = tick_packet
            .clone()
            .players
            .unwrap()
            .get(self.car_id)
            .unwrap()
            .physics
            .clone()
            .unwrap();
        let car_location = car.location.clone().unwrap();
        let rotation = car.rotation.clone().unwrap();
        let velocity = car.velocity.clone().unwrap();
        let car_forward = forward_vec(&rotation);

        let mut shifted_target = self.target.clone();
        let mut shifted_arrival_time = self.arrival_time;
        if let Some(direction) = &self.target_direction {
            let car_speed = velocity.norm();
            let target_direction = direction.normalize();
            // in order to arrive in a direction, we need to shift the target in the opposite direction
            // the magnitude of the shift is based on how far are we from the target
            let mut shift = clamp(
                car_location.ground_dist(&self.target) * self.lerp_t,
                0.,
                clamp(car_speed, 1500., 2300.) * 1.6,
            );

            // if we're too close to the target, aim for the actual target so we don't miss it
            if shift - self.additional_shift * 0.5
                < turn_radius(clamp(car_speed, 500., 2300.)) * 1.1
            {
                shift = 0.;
            } else {
                shift += self.additional_shift;
            }
            shifted_target = self.target.sub(&target_direction.scale(shift));
            let time_shift =
                shifted_target.ground_dist(&self.target) / clamp(car_speed, 500., 2300.) * 1.2;
            shifted_arrival_time = self.arrival_time - time_shift;
        }

        self.drive.target_pos = shifted_target.clone();
        let dist_to_target = car_location.ground_dist(&shifted_target);

        let time_left =
            (shifted_arrival_time - tick_packet.gameInfo.clone().unwrap().secondsElapsed).max(1e-6);
        let mut target_speed = clamp(dist_to_target / time_left, 0., 2300.);
        if target_speed < 800.
            && dist_to_target > 1000.
            && car_forward.angle_between(&shifted_target) < 0.1
        {
            target_speed = 0.;
        }
        self.drive.target_speed = target_speed;

        self.drive.step(tick_packet, controller, dt)
    }

    fn render(&self) -> Vec<rlbot_lib::rlbot::RenderMessage> {
        vec![]
    }

    fn interruptible(&self) -> bool {
        true
    }

    fn kickoff(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        String::from("GotoAction")
    }
}
