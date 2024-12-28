mod account;
mod database;

use::std::io;

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
        println!("0. Exit");

        let mut choice_holder = String::new();
        let mut user_name = String::new();
        let mut password = String::new();

        print!("Enter choice: ");
        if let Err(e) = io::stdin().read_line(&mut choice_holder) {
            println!("Error: {}", e);
        }

        choice = choice_holder.trim().parse().expect(
            "Invalid Integer"
        );

        let mut current_Account : Account;
        

        match choice {
            1 => {
                println!("-----Login Screen-----");
                println!("Enter Name: ");
                io::stdin().read_line(&mut user_name).expect("Invalid Input Entering Name");
                println!("Enter Password: ");
                io::stdin().read_line(&mut password).expect("Invalid Input Entering Password");

                if let Err(e) = database.load(&user_name, &password) {
                    println!("Error: {}", e);
                    continue;
                }
                else {
                    current_Account = database.load(&user_name, &password).unwrap();
                }
            },
            2 => {
                println!("-----Signup Screen-----");
                println!("Enter Name: ");
                io::stdin().read_line(&mut user_name).expect("Invalid Input Entering Name");
                println!("Enter Password: ");
                io::stdin().read_line(&mut password).expect("Invalid Input Entering Password");

                if let Err(e) = database.create_account(&user_name, &password) {
                    println!("Error: {}", e);
                    continue;
                }
                else {
                    current_Account = database.load(&user_name, &password).unwrap();
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

            match user_choice {
                1 => {

                },
                2 => {

                },
                3 => {

                },
                _ => {

                }
            }
        }
    }
    



    database.deinit(&database_file);
}