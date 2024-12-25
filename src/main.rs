mod customer;

use crate::customer::Customer;

fn main() {
    let human = Customer::new();

    print!("name: {}", human.name);
}