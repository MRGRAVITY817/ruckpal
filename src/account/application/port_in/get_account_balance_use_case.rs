use crate::{
    account::application::domain::model::{account::AccountId, money::Money},
    common::result::AppResult,
};

pub trait GetAccountBalanceUseCase {
    fn get_account_balance(
        &self,
        query: GetAccountBalanceQuery,
    ) -> AppResult<Money>;
}

pub struct GetAccountBalanceQuery(AccountId);

impl GetAccountBalanceQuery {
    pub fn account_id(&self) -> AccountId {
        self.0
    }
}
