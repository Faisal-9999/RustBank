mod account;
mod database;

use::std::io;
use std::io::Write;

use crate::database::Database;
use crate::account::Account;

fn main() {
    
    let database_file = "accounts.txt";

    let mut database =  match Database::initialize(&database_file) {
        Ok(data) => data,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let mut choice = -1;

    while choice != 0 {
        println!("Welcome To Bank Management System");
        println!("1. Login");
        println!("2. Signup");
        println!("0. Logout");

        let mut choice_holder = String::new();
        let mut user_name = String::new();
        let mut password = String::new();

        println!("Enter choice: ");
        io::stdout().flush().unwrap();

        if let Err(e) = io::stdin().read_line(&mut choice_holder) {
            println!("Error: {}", e);
        }

        choice = choice_holder.trim().parse().expect(
            "Invalid Integer"
        );

        let mut current_account : Account;

        match choice {
            1 => {
                println!("\n-----Login Screen-----");
                println!("Enter Name: ");
                io::stdin().read_line(&mut user_name).expect("Invalid Input Entering Name");
                println!("Enter Password: ");
                io::stdin().read_line(&mut password).expect("Invalid Input Entering Password");

                if let Err(e) = database.load(&user_name.trim(), &password.trim()) {
                    println!("Error: {}", e);
                    continue;
                }
                else {
                    current_account = database.load(&user_name.trim(), &password.trim()).unwrap();
                }
            },
            2 => {
                println!("-----Signup Screen-----");
                println!("Enter Name: ");
                io::stdin().read_line(&mut user_name).expect("Invalid Input Entering Name");
                println!("Enter Password: ");
                io::stdin().read_line(&mut password).expect("Invalid Input Entering Password");

                if let Err(e) = database.create_account(&user_name.trim(), &password.trim()) {
                    println!("Error: {}", e);
                    continue;
                }
                else {
                    current_account = database.load(&user_name.trim(), &password.trim()).unwrap();
                }
            },
            _ => {
                println!("Invalid Choice");
                continue;
            }
        }

        let mut user_choice = -1;

        while user_choice != 0 {
            println!("----Account Menu-----");
            println!("1. Deposit");
            println!("2. Withdraw");
            println!("3. View Account Details");
            println!("0. Exit");

            let mut choice2 = String::new();

            print!("Enter Choice: ");
            io::stdout().flush().unwrap();
            if let Err(e) = io::stdin().read_line(&mut choice2) {
                println!("Error: {}", e);
                continue;
            }

            user_choice = match choice2.trim().parse() {
                Ok(value) => value,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };


            match user_choice {
                1 => {
                    let mut amount = String::new();
                    println!("Enter Amount To Deposit: ");
                    io::stdout().flush().unwrap();
                    if let Err(e) = io::stdin().read_line(&mut amount) {
                        println!("Error: {}", e);
                    }

                    let to_deposit: i32 = match amount.trim().parse() {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error: {}", e);
                            continue;
                        }
                    };

                    current_account.deposit(to_deposit);
                },
                2 => {
                    let mut amount = String::new();
                    println!("Enter Amount To Withdraw: ");
                    io::stdout().flush().unwrap();
                    if let Err(e) = io::stdin().read_line(&mut amount) {
                        println!("Error: {}", e);
                    }

                    let to_withdraw: i32 = match amount.trim().parse() {
                        Ok(value) => value,
                        Err(e) => {
                            println!("Error: {}", e);
                            continue;
                        }
                    };

                    current_account.withdraw(to_withdraw);
                },
                3 => {
                    current_account.view_account_details();
                },
                0 => {
                    println!("Logging Out");
                    break;
                },
                _ => {
                    println!("Invalid Input");
                }
            }


        }

        database.check_for_edit(current_account);
    }
    

    if let Err(_) = database.deinit(&database_file) {
        panic!("ERROR WHILE DEINITIALIZING DATABASE TO FILE");
    }
}