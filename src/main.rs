use std::ops::Range;

const RAW_OUTPUT_PATH: &str = "pendulum.dat";
const PLOT_OUTPUT_PATH: &str = "pendulum_plot.png";

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
    use std::io::Write;

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
    use plotters::prelude::*;

    // Plot settings
    let dimensions = (640, 502);
    let left_margin = dimensions.0 as f64/ 12.5;
    let bottom_margin = dimensions.1 as f64/ 12.5;
    let right_margin = dimensions.1 as f64/ 10.0;

    let caption = "Pendulum Angle vs Time";
    let caption_font = "Times New Roman";
    let caption_font_size =  75 / 2;
    let caption_style = (caption_font, caption_font_size).into_font();

    let background_color = &WHITE;
    let curve_width = 1;
    let curve_style = RED.stroke_width(curve_width);

    let (x_bounds, y_bounds) = calculate_plot_bounds(&data).ok_or("Couldn't find bounds because data was empty")?;
    let x_axis_description = "Time";
    let y_axis_description = "Angle";
    let axes_description_font = caption_font;
    let axes_description_font_size = 50 / 2;
    let axes_description_style = (axes_description_font, axes_description_font_size).into_font();

    // Create plot file 
    let root = BitMapBackend::new(PLOT_OUTPUT_PATH, dimensions).into_drawing_area();
    root.fill(background_color)?; // set background color

    // Create chart area
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, caption_style) // set caption
        .set_label_area_size(LabelAreaPosition::Bottom, bottom_margin)
        .set_label_area_size(LabelAreaPosition::Left, left_margin)
        .set_label_area_size(LabelAreaPosition::Right, right_margin)
        .build_cartesian_2d(x_bounds, y_bounds)?; // 2d cartesian chart

    // draw axes and grid lines
    // TODO: fix position of descriptions
    chart
        .configure_mesh()
        .x_desc(x_axis_description)
        .y_desc(y_axis_description)
        .axis_desc_style(axes_description_style)
        .draw()?;

    // draw the curve
    chart.draw_series(LineSeries::new(data, curve_style))?;

    root.present()?;

    return Ok(())
}

fn calculate_plot_bounds(data: &[(f64, f64)]) -> Option<(Range<f64>, Range<f64>)> {
    if data.is_empty() {
        return None;
    }

    let y_buffer = 0.5;
    
    let (mut x_min, mut x_max) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut y_min, mut y_max) = (f64::INFINITY, f64::NEG_INFINITY);

    for &(x, y) in data {
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    return Some((x_min..x_max, y_min-y_buffer..y_max+y_buffer));
}

#[test]
fn f() {

}
