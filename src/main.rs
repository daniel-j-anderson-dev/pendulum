use std::f64::consts::FRAC_PI_4;
use glam::DVec2;

pub const GRAVITY: f64 = 9.8;
pub const ITERATION_MAX: usize = 100;
pub const THICKNESS: f64 = 1.0;
pub const MASS_RADIUS: f64 = 10.0;

fn main() {
    let angles: Vec<f64> = Vec::with_capacity(ITERATION_MAX);

    // pendulum
    let length = 150.0;
    let pivot = DVec2::new(0.0, 0.0) * 0.5; // origin of screen
    let mut angle = FRAC_PI_4;
    let mut anglular_acceleration = DVec2::ZERO;
    let mut angular_velocity = DVec2::ZERO;
    let mut angular_position = pivot + DVec2::new(angle.sin(), angle.cos()) * length;
    
    let mut time = 0.0;
    loop {
        // TODO: fix me!!!
        angle += 0.02; 
    
        // update the mass position given the new angle
        mass_position = pivot + DVec2::new(angle.sin(), angle.cos()) * length;
    }
}

fn update_acceleration(angle: f64, length: f64) -> DVec2 {
    
}