pub struct Account {
    account_name: String,
    account_password: String,
    account_balance: i32,
}

impl Account {
    pub fn get_account_name(&self) -> String {
        self.account_name.clone()
    }

    pub fn get_account_balance(&self) -> i32 {
        self.account_balance
    }

    pub fn get_account_password(&self) -> String {
        self.account_password.clone()
    }

    pub fn add_account_balance(&mut self,  amount: i32) {
        self.account_balance += amount;
    }

    pub fn deduct_account_balance(&mut self, amount: i32) {
        self.account_balance -= amount;
    }
}