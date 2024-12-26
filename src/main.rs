mod account;
mod database;

use crate::database::Database;
fn main() {

    let database_file = "accounts.txt";

    let mut database =  match Database::initialize(&database_file) {
        Ok(data) => data,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    
    
}