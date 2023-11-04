use super::common_entity_types::EntityId;
use crate::{
    account::application::domain::model::{
        account::AccountId, activity::Activity, money::Money,
    },
    common::timestamp::Timestamp,
};

pub struct ActivityEntity {
    id: EntityId,
    owner_account_id: i64,
    source_account_id: i64,
    target_account_id: i64,
    timestamp: Timestamp,
    money: i64,
}

impl ActivityEntity {
    pub fn into_activity(self) -> Activity {
        Activity::new(
            AccountId(self.owner_account_id),
            AccountId(self.source_account_id),
            AccountId(self.target_account_id),
            self.timestamp,
            Money(self.money),
        )
    }
}
