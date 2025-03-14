enum FuelType {
Petrol,
Diesel,
Electric,
}

struct Vehicle {
brand: String,
model: String,
fuel_type: FuelType,
}

fn filter_electric_vehicles(vehicles: &Vec<Vehicle>) {
for vehicle in vehicles {
if let FuelType::Electric = vehicle.fuel_type {
println!("Electric Vehicle: {} {}", vehicle.brand, vehicle.model);
}
}
}

fn main() {
let vehicles = vec![
Vehicle { brand: String::from("Tesla"), model: String::from("Model 3"), fuel_type: FuelType::Electric },
Vehicle { brand: String::from("Toyota"), model: String::from("Corolla"), fuel_type: FuelType::Petrol },
Vehicle { brand: String::from("Nissan"), model: String::from("Leaf"), fuel_type: FuelType::Electric },
Vehicle { brand: String::from("Ford"), model: String::from("F-250"), fuel_type: FuelType::Diesel }, // 🚀 Added Diesel vehicle
];

```
filter_electric_vehicles(&vehicles);

```

}
