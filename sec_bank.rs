struct BankAccount {
    balance: f64,
}

fn main() {
    let mut account = BankAccount { balance: 500.0 };

    println!("Balance: {:.2}", view_balance(&account));
    deposit(&mut account, 200.0);
    println!("Balance after deposit: {:.2}", view_balance(&account));

    if withdraw(&mut account, 300.0).is_ok() {
        println!("Withdrawal successful!");
    } else {
        println!("Insufficient balance.");
    }

    println!("Final Balance: {:.2}", view_balance(&account));
}

fn view_balance(account: &BankAccount) -> f64 {
    account.balance
}

fn deposit(account: &mut BankAccount, amount: f64) {
    account.balance += amount;
}

fn withdraw(account: &mut BankAccount, amount: f64) -> Result<(), ()> {
    if account.balance >= amount {
        account.balance -= amount;
        Ok(())
    } else {
        Err(())
    }
}

