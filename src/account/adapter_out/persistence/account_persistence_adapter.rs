use super::common_entity_types::EntityId;
use crate::common::timestamp::Timestamp;

pub struct AccountEntity {
    id: EntityId,
}

pub struct ActivityEntity {
    id: EntityId,
    owner_account_id: i64,
    source_account_id: i64,
    target_account_id: i64,
    timestamp: Timestamp,
    money: i64,
}
