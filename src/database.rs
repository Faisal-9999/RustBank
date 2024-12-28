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

        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        };

        let reader = io::BufReader::new(file);

        for line in reader.lines() { 

            let mut user_name = String::new();
            let mut user_password = String::new();
            let mut user_balance = String::new();

            let line = line.map_err(|e| format!("Error: {}", e))?;

            
            let mut counter = 1;

            for i in line.chars() {
                
                if i == ','  {
                    counter += 1;
                }

                if counter == 1 {
                    user_name.push(i);
                }
                else if counter == 2 {
                    user_password.push(i);
                }
                else {
                    user_balance.push(i);
                }

                let user_balance : i32 =  match user_balance.trim().parse() {
                    Ok(value) => value,
                    Err(e) => {
                        panic!("Error: {}", e);
                    }
                };

                accounts.push(Account::new(user_name.clone(), user_password.clone(), user_balance.clone()));
            }
        }

        Ok(Database{accounts})

    }

    pub fn search(&self, name : &str, password : &str) -> usize {
        for i in 0..self.accounts.len() {
            if self.accounts[i].get_name().as_str() == name && password == self.accounts[i].get_password().as_str() {
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

}