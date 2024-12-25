use std::io::{self, Read};

use crate::account::Account;
use crate::loader::Loader;

pub struct Customer{
    name: String,
    accounts: Vec<Account>,
}

impl ToString for Customer {
    fn to_string(&self) -> String {

        let mut account_names = String::new();

        let mut counter: usize  = 0;

        let result: String = loop {
            if counter == 0 {
                account_names.push_str(&self.accounts[counter].get_account_name());
            }
            else {
                account_names.push_str(&format!(",{}", self.accounts[counter].get_account_name()));
            }

            counter += 1;

            if counter == self.accounts.len() {
                break  (format!("{},{}", &self.name, &account_names));
            }

        };

        result
    }
}

impl Customer {

    //
    //
    // WORK ON THIS AFTER LOADER ONLY NAME WILL BE USED FOR CHECKING IF THE USER EXISTS AND ONLY NAME WILL BE SAVED
    //
    //

    // pub fn login() -> Customer {
    //     let mut name_input = String::new();

    //     //Add check for if data exists in file
    // }

    // pub fn signup() -> Customer {
    //     let mut name_input = String::new();

    //     //add check for it data exists in file
    // }

    pub fn view_accounts(&self) {
        println!("\nYou have {} accounts", self.accounts.len());

        for i in 0..self.accounts.len() {
            println!("{}. Account Name: {}, Account Balance: {}", i + 1, self.accounts[i].get_account_name(), self.accounts[i].get_account_password());
        }

        println!("");
    }

    pub fn deposit(&mut self, amount: i32) -> Result<(), String> {

        if amount < 0 {
            return  Err("Invalid Amount".to_string());
        }

        self.view_accounts();

        let mut input = String::new();
        print!("Enter Choice : (1 - {})", self.accounts.len());
        io::stdin().read_line(&mut input).map_err(|e| format!("Error: {}", e))?;

        let choice = input.trim().parse::<usize>().map_err(|e| format!("Error: {}", e))?;

        if choice > self.accounts.len() {
            return  Err("Account Number Doesn't Exist".to_string());
        }

        println!("Added Balance to Acccount Name: {}", self.accounts[choice].get_account_name());

        self.accounts[choice].add_account_balance(amount);

        println!("Current Account Balance: {}", self.accounts[choice].get_account_balance());

        Ok(())
    }

    pub fn withdraw(&mut self, amount: i32) -> Result<(), String> {
        if amount < 0 {
            return  Err("Invalid Amount".to_string());
        }

        self.view_accounts();

        let mut input = String::new();
        print!("Enter Choice : (1 - {})", self.accounts.len());
        io::stdin().read_line(&mut input).map_err(|e| format!("Error: {}", e))?;

        let choice = input.trim().parse::<usize>().map_err(|e| format!("Error: {}", e))?;

        if choice > self.accounts.len() {
            return  Err("Account Number Doesn't Exist".to_string());
        }

        if amount > self.accounts[choice].get_account_balance() {
            return Err("Insufficient Balance In Account".to_string());
        }

        println!("Deducted Balance From Acccount Name: {}", self.accounts[choice].get_account_name());
        
        self.accounts[choice].deduct_account_balance(amount);

        println!("Current Account Balance: {}", self.accounts[choice].get_account_balance());

        Ok(())
    }
}