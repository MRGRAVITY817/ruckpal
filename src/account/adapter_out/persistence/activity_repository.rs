use super::activity_entity::ActivityEntity;
use crate::{
    account::application::domain::model::account::AccountId,
    common::{result::AppResult, timestamp::Timestamp},
};

pub trait ActivityRepository {
    fn find_by_owner_since(
        &self,
        owner_account_id: AccountId,
        since: Timestamp,
    ) -> AppResult<Vec<ActivityEntity>>;

    fn get_deposit_balance_until(
        &self,
        account_id: AccountId,
        until: Timestamp,
    ) -> AppResult<i64>;

    fn get_withdrawal_balance_until(
        &self,
        account_id: AccountId,
        until: Timestamp,
    ) -> AppResult<i64>;

    fn save_many(&self, activities: Vec<ActivityEntity>) -> AppResult<()>;
}
