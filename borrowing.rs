#[derive(Debug)]
struct BankAccount {
    account_number: String,
    owner_name: String,
    balance: f64,
}

impl BankAccount {
    // Function to display the account details and balance
    fn view_balance(&self) {
        println!("Account Number: {}", self.account_number);
        println!("Owner Name: {}", self.owner_name);
        println!("Balance: ${:.2}", self.balance);
    }

    // Function to deposit money (mutable borrow)
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {} (Owner: {}). New balance: ${:.2}",
                amount, self.account_number, self.owner_name, self.balance
            );
        } else {
            println!("Invalid deposit amount: ${:.2}", amount);
        }
    }

    // Function to withdraw money (mutable borrow)
    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {} (Owner: {}). New balance: ${:.2}",
                amount, self.account_number, self.owner_name, self.balance
            );
        } else {
            println!("Insufficient funds or invalid amount: ${:.2}", amount);
        }
    }
}

fn main() {
    // Create a bank account instance
    let mut account = BankAccount {
        account_number: String::from("987654321"),
        owner_name: String::from("Alice"),
        balance: 1000.0,
    };

    // Display account details and balance
    account.view_balance();

    // Deposit money and display updated details
    account.deposit(250.0);
    account.view_balance();

    // Withdraw money and display updated details
    account.withdraw(100.0);
    account.view_balance();

    // Attempt an invalid withdrawal
    account.withdraw(2000.0);
}

