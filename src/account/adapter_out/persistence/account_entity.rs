use super::{activity_entity::ActivityEntity, common_entity_types::EntityId};
use crate::account::application::domain::model::{
    account::{Account, AccountId},
    activity_window::ActivityWindow,
    money::Money,
};

pub struct AccountEntity {
    pub id: EntityId,
}

impl AccountEntity {
    /// Convert AccountEntity to Account.
    pub fn into_account(
        self,
        deposit_balance: i64,
        withdrawal_balance: i64,
        activities: Vec<ActivityEntity>,
    ) -> Account {
        let baseline_balance = Money(deposit_balance - withdrawal_balance);
        let activity_window = ActivityWindow(
            activities
                .into_iter()
                .map(ActivityEntity::into_activity)
                .collect(),
        );

        Account::with_id(AccountId(self.id), baseline_balance, activity_window)
    }
}
