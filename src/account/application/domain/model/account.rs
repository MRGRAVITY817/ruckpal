use super::{activity::Activity, activity_window::ActivityWindow, money::Money};
use chrono::Local;

pub struct Account {
    id: AccountId,
    base_line_balance: Money,
    activity_window: ActivityWindow,
}

impl Account {
    pub fn new(id: AccountId, base_line_balance: Money, activity_window: ActivityWindow) -> Self {
        Self {
            id,
            base_line_balance,
            activity_window,
        }
    }

    /// Calculate the balance of current account.
    pub fn calculate_balance(&self) -> Money {
        self.base_line_balance + self.activity_window.calculate_balance(self.id)
    }

    /// Withdraw money from current account to target account.
    pub fn withdraw(self, money: Money, target_account_id: AccountId) -> Self {
        if self.may_withdraw(money) {
            return self;
        }

        let withdrawal = Activity::new(self.id, self.id, target_account_id, Local::now(), money);
        let activity_window = self.activity_window.add_activity(withdrawal);

        Self::new(self.id, self.base_line_balance, activity_window)
    }

    /// Deposit money from source account to current account.
    pub fn deposit(self, money: Money, source_account_id: AccountId) -> Self {
        let deposit = Activity::new(self.id, source_account_id, self.id, Local::now(), money);
        let activity_window = self.activity_window.add_activity(deposit);

        Self::new(self.id, self.base_line_balance, activity_window)
    }

    /// Check if account can withdraw the given amount of money.
    fn may_withdraw(&self, money: Money) -> bool {
        (self.calculate_balance() - money).is_positive()
    }
}

#[derive(Clone, Copy)]
pub struct AccountId(i64);
