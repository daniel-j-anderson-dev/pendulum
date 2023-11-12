use std::io::Write;
use std::ops::Range;
use plotters::prelude::*;

const RAW_OUTPUT_PATH: &str = "pendulum_exact.dat";
const PLOT_OUTPUT_PATH: &str = "pendulum_plot.png";
const PLOT_DIMENSIONS: (u32, u32) = (500, 500);
const PLOT_CAPTION: &str = "Pendulum Angle vs Time";
const PLOT_FONT: &str = "Times New Roman";
const PLOT_FONT_SIZE: u32 = 20;
const PLOT_BACKGROUND_COLOR: &RGBColor = &WHITE;
const PLOT_CURVE_COLOR: &RGBColor = &RED;

fn main() {
    let start_time = 0.0;
    let end_time = 20.0;
    let time_step = 0.0002;
    let iteration_max = ((end_time - start_time) / time_step) as usize;
    
    let gravity = 9.81;
    let length = 1.2;
    let mut angular_velocity = 0.0;
    let mut angular_position = 1.0;
    
    let mut data: Vec<(f64, f64)> = Vec::with_capacity(iteration_max); // (time, angular_position)
    
    let mut time = start_time;
    for _ in 1..iteration_max {
        data.push((time, angular_position));

        angular_velocity += -(gravity / length) * angular_position.sin() * time_step;
        angular_position += angular_velocity * time_step;

        time += time_step;
    }

    match save_raw_data(&data) {
        Ok(()) => println!("Saved raw output to {}", RAW_OUTPUT_PATH),
        Err(error) => println!("Error saving output: {}", error),
    }

    match plot_data(data) {
        Ok(()) => println!("Saved output to {}", PLOT_OUTPUT_PATH),
        Err(error) => println!("Error saving output: {}", error),
    }
}

fn save_raw_data(data: &[(f64, f64)]) -> Result<(), std::io::Error> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(RAW_OUTPUT_PATH)?;
    let mut file = std::io::BufWriter::new(file);

    for (time, angle) in data {
        writeln!(file, "{:.16e} {:.16}", time, angle)?;
    }

    return Ok(())
}

fn plot_data(data: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let (x_bounds, y_bounds) = calculate_plot_bounds(&data).ok_or("Couldn't find bounds because data was empty")?;

    let root = BitMapBackend::new(PLOT_OUTPUT_PATH, PLOT_DIMENSIONS).into_drawing_area();
    root.fill(PLOT_BACKGROUND_COLOR)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(PLOT_CAPTION, (PLOT_FONT, PLOT_FONT_SIZE).into_font())
        .build_cartesian_2d(x_bounds, y_bounds)?;
    
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(data, PLOT_CURVE_COLOR))?;

    root.present()?;

    return Ok(())
}

fn calculate_plot_bounds(data: &[(f64, f64)]) -> Option<(Range<f64>, Range<f64>)> {
    if data.is_empty() {
        return None;
    }
    
    let (mut x_min, mut x_max) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut y_min, mut y_max) = (f64::INFINITY, f64::NEG_INFINITY);

    for &(x, y) in data {
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    return Some((x_min..x_max, y_min..y_max));
}