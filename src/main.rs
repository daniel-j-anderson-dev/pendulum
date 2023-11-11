use std::f64::consts::FRAC_PI_4;
use glam::DVec2;

pub const GRAVITY: f64 = 9.8;
pub const ITERATION_MAX: usize = 100;
pub const THICKNESS: f64 = 1.0;
pub const MASS_RADIUS: f64 = 10.0;

fn main() {
    let angles: Vec<f64> = Vec::with_capacity(ITERATION_MAX);

    // pendulum
    let mut anglular_acceleration = 0.0;
    let mut angle = FRAC_PI_4;
    let length = 150.0;
    let pivot = DVec2::new(0.0, 0.0) * 0.5; // origin of screen
    let mut mass_position = pivot + DVec2::new(angle.sin(), angle.cos()) * length;
    
    for time in 0..ITERATION_MAX {
        // TODO: fix me!!!
        angle += 0.02; 

        // update the mass position given the new angle
        mass_position = pivot + DVec2::new(angle.sin(), angle.cos()) * length;
    }
}
