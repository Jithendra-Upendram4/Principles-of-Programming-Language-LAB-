fn main() {
    let temps = [22.5, 23.0, 21.8, 20.2, 19.5, 18.0, 17.5];
    let last_three_days = &temps[4..7];

    println!("Last 3 Days' Temperatures: {:?}", last_three_days);
    println!("Average Temperature: {:.2}", average_temperature(last_three_days));
}

fn average_temperature(temps: &[f32]) -> f32 {
    let sum: f32 = temps.iter().sum();
    sum / temps.len() as f32
}

