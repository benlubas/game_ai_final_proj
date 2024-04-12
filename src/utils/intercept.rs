use rlbot_lib::rlbot::{Physics, PlayerInfo, PredictionSlice, Vector3};

use super::{
    arena::Arena,
    math::math::{forward_vec, Vec3},
};

#[derive(Clone)]
pub struct Intercept {
    pub ball: Physics,
    pub car: PlayerInfo,
    pub is_viable: bool,
    pub time: f32,
    pub location: Vector3,
    use_predicate: bool,
    pub predicate_later_than_time: bool,
}

impl Intercept {
    pub fn new(
        car: &PlayerInfo,
        game_time: f32,
        ball_predictions: &Vec<PredictionSlice>,
        ball: Physics,
        ignore_time_estimate: bool,
        use_predicate: bool,
    ) -> Intercept {
        let mut the_ball: Option<Physics> = None;
        let mut is_viable = true;
        let mut time = f32::MAX;
        let mut predicate_later_than_time = false;
        for ball in ball_predictions.clone() {
            let ball_phys = ball.physics.unwrap();
            let ball_location = ball_phys.location.clone().unwrap();
            time = estimate_time(car, ball_location);

            if time < ball.gameSeconds - game_time || ignore_time_estimate {
                if !use_predicate || predicate(car.clone(), *ball_phys.clone()) {
                    the_ball = Some(*ball_phys.clone());
                    break;
                }
                predicate_later_than_time = true;
            } else {
                predicate_later_than_time = false;
            }
        }
        if the_ball.is_none() {
            if ball_predictions.len() > 0 {
                if let Some(last) = ball_predictions.last() {
                    the_ball = Some(*last.physics.clone().unwrap());
                } else {
                    the_ball = Some(ball.clone());
                }
            }
            is_viable = false;
        }
        let location = Some(the_ball.clone().unwrap().location.unwrap());

        Intercept {
            ball: the_ball.unwrap(),
            car: car.clone(),
            is_viable,
            time,
            location: location.unwrap(),
            use_predicate,
            predicate_later_than_time,
        }
    }
}

pub fn turn_radius(v: f32) -> f32 {
    if v == 0.0 {
        return 0.;
    }
    return 1.0 / curvature(v);
}

// v is the magnitude of the velocity in the car's forward direction
fn curvature(v: f32) -> f32 {
    if 0.0 <= v && v < 500.0 {
        return 0.006900 - 5.84e-6 * v;
    }
    if 500.0 <= v && v < 1000.0 {
        return 0.005610 - 3.26e-6 * v;
    }
    if 1000.0 <= v && v < 1500.0 {
        return 0.004300 - 1.95e-6 * v;
    }
    if 1500.0 <= v && v < 1750.0 {
        return 0.003025 - 1.1e-6 * v;
    }
    if 1750.0 <= v && v < 2500.0 {
        return 0.001800 - 4e-7 * v;
    }
    return 0.0;
}

// This function is ignoring the direction that the car is moving, assuming that it's driving
// forwards
pub fn estimate_time(car: &PlayerInfo, target: Vector3) -> f32 {
    let car_phys = car.physics.clone().unwrap();
    let car_vel = car_phys.velocity.clone().unwrap();
    let car_rot = car_phys.rotation.clone().unwrap();
    let car_loc = car_phys.location.clone().unwrap();
    let forward = forward_vec(&car_rot);
    let turning_radius = turn_radius(car_vel.norm());
    let mut turning = forward.angle_between(&car_loc.direction(&target)) * turning_radius / 1800.;
    if turning < 0.5 {
        turning = 0.;
    }

    let mut dist = car_loc.ground().dist(&target.ground()) - 200.;
    if dist < 0. {
        return turning;
    }

    let mut speed = car_vel.dot(&forward);
    let mut time = 0.;

    if car.boost > 0 {
        let a = 991.666;
        let mut boost_time = car.boost as f32 / 33.33;
        let mut distance_traveled = speed * boost_time + 0.5 * a * boost_time.powi(2);
        if distance_traveled > dist {
            boost_time = ((2. * a * dist + speed.powi(2)).sqrt() - speed) / a;
        }
        speed = (speed + boost_time * a).max(2300.);
        distance_traveled = speed * boost_time + 0.5 * a * boost_time.powi(2);
        time += boost_time;
        dist -= distance_traveled;
    }

    if speed > 0. {
        if dist > 0. && speed < 1410. {
            // we can accelerate with the throttle, but it's inconsistent, the rate of acceleration
            // diminishes are you accelerate. There is no good estimation on the wiki, so I'll just
            // assume that we can't accelerate to faster than we're currently moving I guess (this is
            // a horrible assumption to make)

            time += dist / speed;
            dist = 0.;
        }
        time += dist / speed;
    }
    time * 1.05 + turning
}

// HACK: I'm just passing a boolean to avoid having to pass this function around b/c I can't clone
// it and I don't have time to figure out how to do that. Rust is hard
fn predicate(_car: PlayerInfo, ball: Physics) -> bool {
    let ball_loc = ball.location.clone().unwrap();
    if ball_loc.z > 200. || ball_loc.y.abs() > Arena::SIZE.y - 100. {
        false
    } else {
        // Note: this might be too simplistic to work I'm not sure.
        true
    }
}
