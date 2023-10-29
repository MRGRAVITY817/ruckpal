use crate::{
    account::application::domain::model::{
        account::AccountId,
        money::{is_money_positive, Money},
    },
    common::result::AppResult,
};
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct SendMoneyCommand {
    source_account_id: AccountId,

    target_account_id: AccountId,

    #[validate(custom = "is_money_positive")]
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

    pub fn source_account_id(&self) -> AccountId {
        self.source_account_id
    }

    pub fn target_account_id(&self) -> AccountId {
        self.target_account_id
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
