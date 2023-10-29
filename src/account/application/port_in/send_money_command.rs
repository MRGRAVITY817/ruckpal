use crate::{
    account::application::domain::model::{account::AccountId, money::Money},
    common::AppResult,
};

pub struct SendMoneyCommand {
    source_account_id: AccountId,
    target_account_id: AccountId,
    money: Money,
}

impl SendMoneyCommand {
    pub fn new(
        source_account_id: AccountId,
        target_account_id: AccountId,
        money: Money,
    ) -> AppResult<Self> {
        if !money.is_positive() {
            return Err("cannot send negative amount of money".into());
        }

        Ok(Self {
            source_account_id,
            target_account_id,
            money,
        })
    }
}

// public record SendMoneyCommand(
//         @NotNull AccountId sourceAccountId,
//         @NotNull AccountId targetAccountId,
//         @NotNull @PositiveMoney Money money
// ) {
//
//     public SendMoneyCommand(
//             AccountId sourceAccountId,
//             AccountId targetAccountId,
//             Money money) {
//         this.sourceAccountId = sourceAccountId;
//         this.targetAccountId = targetAccountId;
//         this.money = money;
//         validate(this);
//     }
//
// }