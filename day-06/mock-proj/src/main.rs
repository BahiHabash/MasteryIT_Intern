mod bank;
mod logger;
mod person;

use person::Person;
use bank::BankAccount;
use logger::{Logger, ConsoleLogger};

fn main() {
    let logger = ConsoleLogger{};
    let customer = Person {
        first_name: String::from("Bahi"), 
        last_name: String::from("Habash"), 
        age: 22
    };
    let mut bank_account = BankAccount::new(
        50, 
        customer
    );

    logger.log(&format!("Init Account: {:?}", bank_account));

    match bank_account.withdraw(200) {
        Ok(balance) => logger.log(&format!("Curr Balance: {balance}")),
        Err(e) => logger.log(&e.to_string()),
    }

    match bank_account.deposite(500) {
        Ok(balance) => logger.log(&format!("Curr Balance: {balance}")),
        Err(e) => logger.log(&e.to_string()),
    }

    logger.log(&format!("Final Balance: {}", bank_account.balance()));
}