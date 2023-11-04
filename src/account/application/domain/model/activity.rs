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

impl Activity {
    /// Creates a new [`Activity`].
    pub fn new(
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
// public class Activity {
//
// 	@Getter
// 	private ActivityId id;
//
// 	/**
// 	 * The account that owns this activity.
// 	 */
// 	@Getter
// 	@NonNull
// 	private final Account.AccountId ownerAccountId;
//
// 	/**
// 	 * The debited account.
// 	 */
// 	@Getter
// 	@NonNull
// 	private final Account.AccountId sourceAccountId;
//
// 	/**
// 	 * The credited account.
// 	 */
// 	@Getter
// 	@NonNull
// 	private final Account.AccountId targetAccountId;
//
// 	/**
// 	 * The timestamp of the activity.
// 	 */
// 	@Getter
// 	@NonNull
// 	private final LocalDateTime timestamp;
//
// 	/**
// 	 * The money that was transferred between the accounts.
// 	 */
// 	@Getter
// 	@NonNull
// 	private final Money money;
//
// 	public Activity(
// 			@NonNull Account.AccountId ownerAccountId,
// 			@NonNull Account.AccountId sourceAccountId,
// 			@NonNull Account.AccountId targetAccountId,
// 			@NonNull LocalDateTime timestamp,
// 			@NonNull Money money) {
// 		this.id = null;
// 		this.ownerAccountId = ownerAccountId;
// 		this.sourceAccountId = sourceAccountId;
// 		this.targetAccountId = targetAccountId;
// 		this.timestamp = timestamp;
// 		this.money = money;
// 	}
//
// 	@Value
// 	public static class ActivityId {
// 		private final Long value;
// 	}
//
// }
