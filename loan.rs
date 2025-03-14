use std::io;

fn main() {
let mut age = String::new();
let mut income = String::new();

```
println!("Enter your age:");
io::stdin().read_line(&mut age).expect("Failed to read input");
let age: u32 = age.trim().parse().expect("Invalid input");

println!("Enter your monthly income:");
io::stdin().read_line(&mut income).expect("Failed to read input");
let income: u32 = income.trim().parse().expect("Invalid input");

if age < 21 {
    println!("You are not eligible for a loan.");
} else if age >= 21 && age <= 60 {
    if income > 50000 {
        println!("You are eligible for a loan.");
    } else {
        println!("You need an income above ₹50,000 to be eligible.");
    }
} else {
    println!("You need a guarantor for a loan.");
}

```

}
