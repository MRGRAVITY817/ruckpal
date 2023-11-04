use super::common_entity_types::EntityId;
use crate::{
    account::application::domain::model::{
        account::AccountId,
        activity::{Activity, ActivityId},
        money::Money,
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
        Activity {
            id: ActivityId(self.id),
            owner_account_id: AccountId(self.owner_account_id),
            source_account_id: AccountId(self.source_account_id),
            target_account_id: AccountId(self.target_account_id),
            timestamp: self.timestamp,
            money: Money(self.money),
        }
    }
}

impl From<Activity> for ActivityEntity {
    fn from(value: Activity) -> Self {
        Self {
            id: value.id.0,
            owner_account_id: value.owner_account_id.0,
            source_account_id: value.source_account_id.0,
            target_account_id: value.target_account_id.0,
            timestamp: value.timestamp,
            money: value.money.0,
        }
    }
}
