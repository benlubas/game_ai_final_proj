const LOCATIONS: [(f32, f32, f32); 34] = [
    (0.0, -4240.0, 70.0),
    (-1792.0, -4184.0, 70.0),
    (1792.0, -4184.0, 70.0),
    (-3072.0, -4096.0, 73.0),
    (3072.0, -4096.0, 73.0),
    (-940.0, -3308.0, 70.0),
    (940.0, -3308.0, 70.0),
    (0.0, -2816.0, 70.0),
    (-3584.0, -2484.0, 70.0),
    (3584.0, -2484.0, 70.0),
    (-1788.0, -2300.0, 70.0),
    (1788.0, -2300.0, 70.0),
    (-2048.0, -1036.0, 70.0),
    (0.0, -1024.0, 70.0),
    (2048.0, -1036.0, 70.0),
    (-3584.0, 0.0, 73.0),
    (-1024.0, 0.0, 70.0),
    (1024.0, 0.0, 70.0),
    (3584.0, 0.0, 73.0),
    (-2048.0, 1036.0, 70.0),
    (0.0, 1024.0, 70.0),
    (2048.0, 1036.0, 70.0),
    (-1788.0, 2300.0, 70.0),
    (1788.0, 2300.0, 70.0),
    (-3584.0, 2484.0, 70.0),
    (3584.0, 2484.0, 70.0),
    (0.0, 2816.0, 70.0),
    (-940.0, 3310.0, 70.0),
    (940.0, 3308.0, 70.0),
    (-3072.0, 4096.0, 73.0),
    (3072.0, 4096.0, 73.0),
    (-1792.0, 4184.0, 70.0),
    (1792.0, 4184.0, 70.0),
    (0.0, 4240.0, 70.0),
];
pub mod pads {
    use rlbot_lib::rlbot::{GameTickPacket, PlayerInfo, Vector3};

    use crate::utils::{
        intercept::estimate_time,
        math::math::{vec_new, Vec3},
    };

    use super::LOCATIONS;

    pub struct BoostPad {
        pub is_active: bool,
        pub timer: f32,
        pub location: Vector3,
    }

    impl BoostPad {
        pub fn extract_info(tick_packet: &GameTickPacket) -> Vec<BoostPad> {
            tick_packet
                .boostPadStates
                .clone()
                .unwrap()
                .into_iter()
                .enumerate()
                .map(|(i, pad)| BoostPad {
                    location: vec_new(LOCATIONS[i].0, LOCATIONS[i].1, LOCATIONS[i].2),
                    timer: pad.timer,
                    is_active: pad.isActive,
                })
                .collect()
        }
    }

    impl PartialEq for BoostPad {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location
        }
    }

    pub fn choose_boostpad(
        info: &GameTickPacket,
        car: PlayerInfo,
        my_goal: &Vector3,
        bad_pads: Vec<BoostPad>,
    ) -> Option<BoostPad> {
        let pads = BoostPad::extract_info(&info);
        let active_pads = pads.into_iter().filter(|pad| {
            pad.is_active || estimate_time(&car, pad.location.clone()) * 0.7 > pad.timer
        });

        let valid_pads = active_pads.filter(|pad| !bad_pads.contains(pad));
        let mut valid_pads = valid_pads.peekable();
        if valid_pads.peek().is_none() {
            return None;
        }

        // Choose the pad that's closest to the midpoint of us, the ball, and our goal, weighting
        // our position and the goals position 2x higher than the ball

        let pos = info
            .ball
            .clone()
            .unwrap()
            .physics
            .unwrap()
            .location
            .unwrap()
            .add(&car.physics.clone().unwrap().location.unwrap().scale(2.))
            .add(&my_goal.scale(2.))
            .scale(0.2);

        valid_pads.min_by(|pad1, pad2| {
            (pad1
                .location
                .dist(&pos)
                .partial_cmp(&pad2.location.dist(&pos)))
            .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
