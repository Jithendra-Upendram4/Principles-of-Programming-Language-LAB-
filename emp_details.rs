use std::io;

fn apply_salary_hike(employee: (u32, String, f64)) -> (u32, String, f64) {
let (id, name, salary) = employee;
let new_salary = if salary < 50000.0 { salary * 1.1 } else { salary };
(id, name, new_salary)
}

fn main() {
let mut id = String::new();
let mut name = String::new();
let mut salary = String::new();

```
// Getting user input
println!("Enter Employee ID:");
io::stdin().read_line(&mut id).expect("Failed to read input");
let id: u32 = id.trim().parse().expect("Invalid ID input");

println!("Enter Employee Name:");
io::stdin().read_line(&mut name).expect("Failed to read input");
let name = name.trim().to_string();

println!("Enter Employee Salary:");
io::stdin().read_line(&mut salary).expect("Failed to read input");
let salary: f64 = salary.trim().parse().expect("Invalid salary input");

let employee = (id, name, salary);
let updated_employee = apply_salary_hike(employee);

println!("\\nUpdated Employee Details:");
println!("ID: {}", updated_employee.0);
println!("Name: {}", updated_employee.1);
println!("Salary: ₹{:.2}", updated_employee.2);

```

}
