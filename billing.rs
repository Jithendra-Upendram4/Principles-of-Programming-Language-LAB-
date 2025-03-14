use std::io;

fn main() {
let mut item = String::new();
let mut quantity = String::new();

```
println!("Enter the menu item (Burger, Pizza, Pasta):");
io::stdin().read_line(&mut item).expect("Failed to read input");
let item = item.trim();

println!("Enter the quantity:");
io::stdin().read_line(&mut quantity).expect("Failed to read input");
let quantity: u32 = quantity.trim().parse().expect("Invalid input");

let price = match item {
    "Burger" => 150,
    "Pizza" => 300,
    "Pasta" => 250,
    _ => {
        println!("Invalid item.");
        return;
    }
};

let total_price = if quantity >= 3 { (price * quantity) as f32 * 0.9 } else { (price * quantity) as f32 };

println!("Total cost for {} {}(s): ₹{:.2}", quantity, item, total_price);

```

}
