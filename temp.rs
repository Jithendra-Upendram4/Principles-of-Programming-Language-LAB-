fn average_temperature(temps: &Vec<f32>) -> f32 {
let sum: f32 = temps.iter().sum();
sum / temps.len() as f32
}

fn min_max_temperature(temps: &Vec<f32>) -> (f32, f32) {
let min = temps.iter().cloned().fold(f32::INFINITY, f32::min);
let max = temps.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
(min, max)
}

fn main() {
let temperatures = vec![30.5, 32.0, 29.8, 31.2, 28.5, 27.9, 33.1];

```
println!("Average Temperature: {:.2}", average_temperature(&temperatures));
let (min, max) = min_max_temperature(&temperatures);
println!("Lowest: {:.2}, Highest: {:.2}", min, max);

```

}
