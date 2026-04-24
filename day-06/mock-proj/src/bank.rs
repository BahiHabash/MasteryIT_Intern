use rand::Rng;
use std::cmp::max;
use std::fmt;
use crate::person::Person;

pub enum BankError {
    InsufficientFunds,
    InvalidAmount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BankError::InsufficientFunds => write!(f, "Error: Insufficient funds."),
            BankError::InvalidAmount => write!(f, "Error: Invalid amount. Must be positive."),
        }
    }
}

#[derive(Debug)]
pub struct BankAccount {
    balance: u64,
    pub account_number: u32,
    pub owner: Person,
}

impl BankAccount {
    pub fn new(balance: u64, owner: Person) -> Self {
        BankAccount {
            balance: max(balance, 0),
            owner,
            account_number: BankAccount::generate_account_number(),
        }
    }

    pub fn deposite(&mut self, amount: u64) -> Result<u64, BankError> {
        if amount <= 0 {
            return Err(BankError::InvalidAmount);
        }

        self.balance += amount;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<u64, BankError> {
        match self.balance.checked_sub(amount) {
            Some(new_balance) => {
                self.balance = new_balance;
                Ok(self.balance)
            },
            None => Err(BankError::InsufficientFunds),
        }
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    fn generate_account_number() -> u32 {
        let mut rng = rand::rng();
        rng.random_range(1_000_000..9_999_999)
    }
}
