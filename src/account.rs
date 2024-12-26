pub struct Account {
    name : String,
    password : String,
    balance : i32,
}

impl Account {
    pub fn new(name : String, password : String, balance : i32) -> Account {
        Account {
            name,
            password,
            balance,
        }
    }

    pub fn deposit(&mut self, amount : i32) {
        if amount < 0 {
            println!("Invalid Amount");
            return;
        }

        println!("Previous Balance: {}", self.balance);

        self.balance += amount;

        println!("Current Balance: {}", self.balance);
    }

    pub fn withdraw(&mut self, amount : i32) {
        if amount < 0 || amount > self.balance {
            println!("Invalid Balance");
            return;
        }

        println!("Previous Balance: {}", self.balance);

        self.balance -= amount;

        println!("Current Balance: {}", self.balance);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }
}