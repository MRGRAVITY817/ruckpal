use super::{account::AccountId, activity::Activity, money::Money};

#[derive(Clone)]
pub struct ActivityWindow(pub Vec<Activity>);

impl Default for ActivityWindow {
    fn default() -> Self {
        Self(vec![])
    }
}

impl ActivityWindow {
    pub fn get_activities(&self) -> &[Activity] {
        self.0.as_slice()
    }

    pub fn calculate_balance(&self, account_id: AccountId) -> Money {
        Money(0)
    }

    pub fn add_activity(self, activity: Activity) -> Self {
        Self(self.0.into_iter().chain([activity]).collect())
    }
}
