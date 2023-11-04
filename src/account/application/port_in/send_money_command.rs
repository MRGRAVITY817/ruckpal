use crate::{
    account::application::domain::model::{
        account::AccountId,
        money::{is_money_positive, Money},
    },
    common::result::{AppError, AppResult},
};
use validator::Validate;

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
        let cmd = Self {
            source_account_id,
            target_account_id,
            money,
        };

        cmd.validate()
            .or(Err(AppError::FieldInputInvalid))
            .and_then(|_| Ok(cmd))
    }

    pub fn source_account_id(&self) -> AccountId {
        self.source_account_id
    }

    pub fn target_account_id(&self) -> AccountId {
        self.target_account_id
    }
}
