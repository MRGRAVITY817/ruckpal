use crate::account::application::domain::model::account::AccountId;
use chrono::{DateTime, Local};

pub trait LoadAccountPort {
    fn load_account(&self, account_id: AccountId, baseline_date: DateTime<Local>);
}
