struct BankAccount {
account_number: u32,
holder_name: String,
balance: f64,
}

impl BankAccount {
fn deposit(&mut self, amount: f64) {
self.balance += amount;
println!("Deposited ₹{:.2}. New Balance: ₹{:.2}", amount, self.balance);
}

```
fn withdraw(&mut self, amount: f64) {
    if amount <= self.balance {
        self.balance -= amount;
        println!("Withdrawn ₹{:.2}. New Balance: ₹{:.2}", amount, self.balance);
    } else {
        println!("Insufficient balance!");
    }
}

fn display(&self) {
    println!("Account {}: {}, Balance: ₹{:.2}", self.account_number, self.holder_name, self.balance);
}

```

}

fn main() {
let mut account = BankAccount {
account_number: 12345,
holder_name: String::from("John Doe"),
balance: 10000.0,
};

```
account.display();
account.deposit(5000.0);
account.withdraw(2000.0);
account.display();

```

}
