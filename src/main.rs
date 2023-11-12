use std::{
    fs::OpenOptions,
    io::{
        BufWriter,
        Write,
    },
};

const RAW_OUTPUT_PATH: &str = "pendulum_exact.dat";

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

    match save_data(&data) {
        Ok(()) => println!("Saved output to {}", RAW_OUTPUT_PATH),
        Err(error) => println!("Error saving output: {}", error),
    }
}

fn save_data(data: &[(f64, f64)]) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(RAW_OUTPUT_PATH)?;
    let mut file = BufWriter::new(file);

    for (time, angle) in data {
        writeln!(file, "{:.16e} {:.16}", time, angle)?;
    }

    return Ok(())
}

