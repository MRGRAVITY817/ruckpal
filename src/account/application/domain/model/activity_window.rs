use super::{account::AccountId, money::Money};

pub struct ActivityWindow;

impl ActivityWindow {
    pub fn calculate_balance(&self, account_id: AccountId) -> Money {
        Money(0)
    }
}
