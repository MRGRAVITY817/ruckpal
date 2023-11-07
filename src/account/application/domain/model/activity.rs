use super::{account::AccountId, money::Money};
use crate::common::timestamp::Timestamp;
use chrono::{DateTime, Local};

#[derive(Clone, Copy)]
pub struct Activity {
    pub id: ActivityId,
    pub owner_account_id: AccountId,
    pub source_account_id: AccountId,
    pub target_account_id: AccountId,
    pub timestamp: Timestamp,
    pub money: Money,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            id: ActivityId(0),
            owner_account_id: AccountId(0),
            source_account_id: AccountId(0),
            target_account_id: AccountId(0),
            timestamp: Local::now(),
            money: Money(0),
        }
    }
}

impl Activity {
    /// Creates a new [`Activity`].
    pub fn without_id(
        owner_account_id: AccountId,
        source_account_id: AccountId,
        target_account_id: AccountId,
        timestamp: DateTime<Local>,
        money: Money,
    ) -> Self {
        Self {
            id: ActivityId(0),
            owner_account_id,
            source_account_id,
            target_account_id,
            timestamp,
            money,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ActivityId(pub i64);
