use crate::account::Account;

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub struct Database {
    accounts : Vec<Account>
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

    //IGNORE EVERYTHING ELSE YOU FIXED THE DATABASE PROBLEM BY MAKING THE DATABASE A VECTOR THAT INITIALIZES BY LOADING ONCE FROM THE FILE
    //YOU ARE MAKING THE SEARCH FUNCTION FOR BOTH UNIVERSAL CHECKS OF LOGIN AND SIGNUP
    //IN LOGIN YOU WILL USE IT TO CHECK IF THE DATA EXISTS AND FOR SIGNUP YOU CAN MAKE SURE THE DATA ISNT ALREADY TAKEN
    //THE LOAD FUNCTION WILL LOAD AN ACCOUNT AFTER THE CHECK FOR SEARCH IS DONE AND IS SUCCESSFUL SO YES THE SEARCH FUNCTION WILL ALSO BE USED IN THE FUNCTION BELOW
    //AND THE SEARCH FUNCTION CAN ALSO BE USED FOR WHEN THE USER IS SIGNING UP

    pub fn load(&self, name : &str, password : &str) -> Result<Account, String> {

    }

    //this is basically the signup function this will also use the search function before hand the check
    // if user data already exists if it does just stop them from making another account
    pub fn create_account(&self, name : &str, password : &str) -> Result<Account, String> {

    }

    //this function will be called at the end to make sure once the program has ended and the user has decided to exit we save
    //the database struct contents into the file
    pub fn deinit(&self, file_name : &str) {

    }

}