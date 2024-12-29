use crate::account::Account;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::io::{self, BufRead};

pub struct Database {
    accounts : Vec<Account>,
}

impl Database {

    pub fn initialize(file_name : &str) -> Result<Database, String> {
        let mut accounts = Vec::new();

        if !Path::new(file_name).exists() {
            if let Err(e) = File::create(file_name) {
                return Err(format!("Error: {}", e));
            }
            
            println!("Successfully Created Database File");
        }

        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        };

        let reader = io::BufReader::new(file);

        for line in reader.lines() { 

            let line = line.map_err(|e| format!("Error: {}", e))?;

            let mut parts = line.splitn(3, ',');


            let user_name = parts.next().unwrap_or_default().to_string();
            let user_password = parts.next().unwrap_or_default().to_string();
            let user_balance = parts.next().unwrap_or_default().trim();

            let user_balance : i32 = user_balance.parse().map_err(|e| format!("Error: {}", e))?;

            accounts.push(Account::new(user_name, user_password, user_balance))
        }

        Ok(Database{accounts})

    }

    pub fn search(&self, name : &str, password : &str) -> usize {
        for i in 0..self.accounts.len() {
            if self.accounts[i].get_name().trim() == name && password == self.accounts[i].get_password().trim() {
                return i;
            }
        }

        99999999999999999
    }

    pub fn load(&self, name : &str, password : &str) -> Result<Account, String> {

        let check: usize = self.search(name, password);
    
        if check == 99999999999999999 {
            return Err("Error: Can't Load... Account Data Doesn't Exist".to_string());
        }

        Ok(self.accounts[check].clone())
    }

    pub fn create_account(&mut self, name : &str, password : &str) -> Result<Account, String> {
        if self.search(name, password) !=  99999999999999999 {
            return Err("Data already exists".to_string());
        }

        if name.is_empty() {
            return Err("Name is empty".to_string());
        }

        if password.is_empty() {
            return Err("Password is empty".to_string());
        }   

        if password.len() < 8 {
            return Err("Password length can't be less than 8".to_string());
        }

        self.accounts.push(Account::new(name.to_string(), password.to_string(), 0));

        Ok(Account::new(name.to_string(), password.to_string(), 0))
    }

    pub fn deinit(&self, file_name : &str) -> Result<(), io::Error>{
        let mut file = OpenOptions::new().write(true).truncate(true).open(file_name)?;

        let accounts_holder = &self.accounts;

        for account  in accounts_holder {
            writeln!(file, "{}", account.to_string())?;
        }

        println!("Database Saved Successfully");

        Ok(())
    }

    pub fn check_for_edit(&mut self, account : Account) {
        for data in &mut self.accounts {
            if account.get_name() == data.get_name() && account.get_password() == data.get_password() {
                data.set_balance(account.get_balance());
            }
        }
    }

}