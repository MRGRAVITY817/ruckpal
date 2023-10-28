use super::{activity_window::ActivityWindow, money::Money};

pub struct Account {
    id: AccountId,
    base_line_balance: Money,
    activity_window: ActivityWindow,
}

impl Account {
    pub fn calculate_balance(&self) -> Money {
        self.base_line_balance + self.activity_window.calculate_balance(self.id)
    }
}

pub struct AccountId(i64);
