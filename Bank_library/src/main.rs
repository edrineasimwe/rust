use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum AccountType {
    Savings,
    Current,
}

#[derive(Debug, Clone)]
struct Bank {
    account_number: u32,
    account_name: String,
    balance: i32,
    pin: u32,
    transactions: Vec<String>,
    account_type: AccountType,
}

trait BankTrait {
    fn create_account(account_name: String, account_number: u32, pin: u32, account_type: AccountType) -> Self;
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32, pin: u32) -> Result<(), &'static str>;
    fn balance(&self, pin: u32) -> Result<i32, &'static str>;
    fn account_details(&self) -> (u32, String, AccountType);
    fn transfer(&mut self, amount: u32, recipient: &mut Bank, pin: u32) -> Result<(), &'static str>;
}

impl BankTrait for Bank {
    fn create_account(account_name: String, account_number: u32, pin: u32, account_type: AccountType) -> Self {
        Bank {
            account_number,
            account_name,
            balance: 0,
            pin,
            transactions: Vec::new(),
            account_type,
        }
    }

    fn deposit(&mut self, amount: u32) {
        self.balance += amount as i32;
        self.transactions.push(format!("Deposited: ${}", amount));
    }

    fn withdraw(&mut self, amount: u32, pin: u32) -> Result<(), &'static str> {
        if self.pin != pin {
            return Err("Incorrect PIN");
        }
        if self.balance >= amount as i32 {
            self.balance -= amount as i32;
            self.transactions.push(format!("Withdrew: ${}", amount));
            Ok(())
        } else {
            Err("Insufficient Balance")
        }
    }

    fn balance(&self, pin: u32) -> Result<i32, &'static str> {
        if self.pin == pin {
            Ok(self.balance)
        } else {
            Err("Incorrect PIN")
        }
    }

    fn account_details(&self) -> (u32, String, AccountType) {
        (self.account_number, self.account_name.clone(), self.account_type.clone())
    }

    fn transfer(&mut self, amount: u32, recipient: &mut Bank, pin: u32) -> Result<(), &'static str> {
        if self.pin != pin {
            return Err("Incorrect PIN");
        }
        if self.balance >= amount as i32 {
            self.balance -= amount as i32;
            recipient.balance += amount as i32;
            self.transactions.push(format!("Transferred: ${} to {}", amount, recipient.account_name));
            recipient.transactions.push(format!("Received: ${} from {}", amount, self.account_name));
            Ok(())
        } else {
            Err("Insufficient Balance")
        }
    }
}

fn main() {
    let mut yvan_account = Bank::create_account(String::from("Yvan"), 1234, 4321, AccountType::Savings);
    let mut ayo_account = Bank::create_account(String::from("Ayo"), 5678, 1234, AccountType::Current);

    println!("New account created: {:#?}", yvan_account);
    
    yvan_account.deposit(500);
    println!("Deposited $500. New balance: {:?}", yvan_account.balance(4321));

    match yvan_account.withdraw(200, 4321) {
        Ok(_) => println!("Withdrawal successful. New balance: {:?}", yvan_account.balance(4321)),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    match yvan_account.transfer(100, &mut ayo_account, 4321) {
        Ok(_) => {
            println!("Transfer successful.");
            println!("Yvan's New Balance: {:?}", yvan_account.balance(4321));
            println!("Ayo's New Balance: {:?}", ayo_account.balance(1234));
        }
        Err(e) => println!("Transfer failed: {}", e),
    }

    println!("Yvan's Transactions: {:?}", yvan_account.transactions);
    println!("Ayo's Transactions: {:?}", ayo_account.transactions);
}
