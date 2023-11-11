use std::f32::consts::FRAC_PI_4;
use macroquad::prelude::*;

pub const GRAVITY: f64 = 9.8;
pub const ITERATION_MAX: usize = 100;
pub const THICKNESS: f32 = 1.0;
pub const MASS_RADIUS: f32 = 10.0;

#[macroquad::main("pendulum")]
async fn main() {
    // pendulum
    let mut angle = FRAC_PI_4;
    let length = 150.0;
    let pivot = Vec2::new(screen_width(), screen_height()) * 0.5; // origin of screen
    let mut mass_position = pivot + Vec2::new(angle.sin(), angle.cos()) * length;
    
    loop {
        clear_background(LIGHTGRAY);

        // TODO: fix me!!!
        angle += 0.01; 

        // update the mass position given the new angle
        mass_position = pivot + Vec2::new(angle.sin(), angle.cos()) * length;

        // draw pendulum
        draw_line(
            pivot.x, pivot.y,
            mass_position.x, mass_position.y,
            THICKNESS, BLACK
        );
        draw_circle(
            mass_position.x, mass_position.y,
            MASS_RADIUS, BLACK
        );

        next_frame().await;
    }
}
