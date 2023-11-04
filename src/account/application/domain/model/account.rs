use super::{
    activity::Activity, activity_window::ActivityWindow, money::Money,
};
use chrono::Local;
use serde::Deserialize;

pub struct Account {
    id: AccountId,
    baseline_balance: Money,
    activity_window: ActivityWindow,
}

impl Account {
    pub fn without_id(
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> Self {
        Self {
            id: AccountId::default(),
            baseline_balance,
            activity_window,
        }
    }

    pub fn with_id(
        id: AccountId,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> Self {
        Self {
            id,
            baseline_balance,
            activity_window,
        }
    }

    /// Calculate the balance of current account.
    pub fn calculate_balance(&self) -> Money {
        self.baseline_balance + self.activity_window.calculate_balance(self.id)
    }

    /// Withdraw money from current account to target account.
    pub fn withdraw(self, money: Money, target_account_id: AccountId) -> Self {
        if self.may_withdraw(money) {
            return self;
        }

        let withdrawal = Activity::new(
            self.id,
            self.id,
            target_account_id,
            Local::now(),
            money,
        );
        let activity_window = self.activity_window.add_activity(withdrawal);

        Self::with_id(self.id, self.baseline_balance, activity_window)
    }

    /// Deposit money from source account to current account.
    pub fn deposit(self, money: Money, source_account_id: AccountId) -> Self {
        let deposit = Activity::new(
            self.id,
            source_account_id,
            self.id,
            Local::now(),
            money,
        );
        let activity_window = self.activity_window.add_activity(deposit);

        Self::with_id(self.id, self.baseline_balance, activity_window)
    }

    /// Check if account can withdraw the given amount of money.
    fn may_withdraw(&self, money: Money) -> bool {
        (self.calculate_balance() - money).is_positive()
    }
}

#[derive(Clone, Copy, Deserialize, Default)]
pub struct AccountId(pub i64);
