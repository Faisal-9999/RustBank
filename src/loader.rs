use std::io;

use crate::customer::Customer;
use crate::account::Account;

pub struct Loader {
    
}

impl Loader {

    //ADD WORKING FOR THIS THEN PROCEED TO customer class for login and signup working
    pub fn check_data_exists(data : &str, file_name : &str) -> bool {
        

        false
    }

    pub fn save_data(file_name : &str) -> Result<(), String> {

        Ok(())
    }

    pub fn load_customer_data(file_name : &str) -> Result<Customer, String> {



        Ok()
    }

    pub fn load_account(account_name : &str) -> Result<Account, String> {

        Ok()
    }


}