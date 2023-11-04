use super::{account::AccountId, activity::Activity, money::Money};

pub struct ActivityWindow(pub Vec<Activity>);

impl ActivityWindow {
    pub fn calculate_balance(&self, account_id: AccountId) -> Money {
        Money(0)
    }

    pub fn add_activity(self, activity: Activity) -> Self {
        Self(self.0.into_iter().chain([activity]).collect())
    }
}
