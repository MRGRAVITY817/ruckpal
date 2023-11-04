use {
    super::account_persistence_adapter::ActivityEntity,
    crate::{
        account::application::domain::model::account::AccountId,
        common::timestamp::Timestamp,
    },
};

pub trait ActivityRepository {
    fn find_by_owner_since(
        owner_account_id: AccountId,
        since: Timestamp,
    ) -> Vec<ActivityEntity>;

    fn get_deposit_balance_unit(
        accound_id: AccountId,
        until: Timestamp,
    ) -> Option<i64>;

    fn get_withdrawal_balance_until(
        account_id: AccountId,
        until: Timestamp,
    ) -> Option<i64>;
}
