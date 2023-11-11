use std::f64::consts::FRAC_PI_4;
use glam::DVec2;

pub const GRAVITY: f64 = 9.81;
pub const THICKNESS: f64 = 1.0;
pub const MASS_RADIUS: f64 = 10.0;
pub const DELTA_TIME: f64 = 0.001;
pub const START_TIME: f64 = 0.0;
pub const END_TIME: f64 = 100.0;

fn main() {
    let angles: Vec<f64> = Vec::with_capacity((END_TIME - START_TIME) as usize);

    // pendulum
    let length = 150.0;
    let pivot = DVec2::new(0.0, 0.0) * 0.5;
    let mut angle = FRAC_PI_4;
    let mut anglular_acceleration = DVec2::ZERO;
    let mut angular_velocity = DVec2::ZERO;
    let mut angular_position = pivot + DVec2::new(angle.sin(), angle.cos()) * length;
    
    let mut time = START_TIME;
    while time < END_TIME {
        // TODO: fix me!!!
        angle += 0.02; 
    
        // update the mass position given the new angle
        angular_position = pivot + DVec2::new(angle.sin(), angle.cos()) * length;

        time += DELTA_TIME;
    }
}

fn update_acceleration(angle: f64, length: f64) -> f64 {
    return (angle.sin()) * (GRAVITY / length);
}