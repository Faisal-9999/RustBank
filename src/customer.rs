mod account;

use account::Account;

pub struct Customer{

    name: String,
    accounts: Vec<Account>,
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            name : "Default".to_string(),
            accounts: vec![],
        }
    }
}