pub struct Account {
    name : String,
    password : String,
    balance : i32,
}

impl Clone for Account {
    fn clone(&self) -> Account {
        Account {
            name : self.name.clone(),
            password : self.password.clone(),
            balance : self.balance,
        }
    }
}

impl ToString for Account {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.name.trim(), self.password.trim(), self.balance)
    }
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

    pub fn view_account_details(&self) {
        println!(
            "NAME: {}\nPASSWORD: {}\nBALANCE: {}", self.name, self.password, self.balance
        )
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

    pub fn set_balance(&mut self, amount : i32) {
        self.balance = amount
    }
}