use crate::{
    account::application::{
        domain::model::money::Money,
        port_in::get_account_balance_use_case::{
            GetAccountBalanceQuery, GetAccountBalanceUseCase,
        },
        port_out::load_account_port::LoadAccountPort,
    },
    common::result::AppResult,
    inject,
};
use chrono::Local;

pub struct GetAccountBalance {
    load_account_port: inject!(LoadAccountPort),
}

impl GetAccountBalanceUseCase for GetAccountBalance {
    fn get_account_balance(
        &self,
        query: GetAccountBalanceQuery,
    ) -> AppResult<Money> {
        self.load_account_port
            .load_account(query.account_id(), Local::now())
            .and_then(|account| Ok(account.calculate_balance()))
    }
}
