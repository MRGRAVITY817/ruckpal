use crate::{
    account::application::domain::model::account::{Account, AccountId},
    common::result::AppResult,
};
use chrono::{DateTime, Local};

pub trait LoadAccountPort {
    fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: DateTime<Local>,
    ) -> AppResult<Account>;
}
