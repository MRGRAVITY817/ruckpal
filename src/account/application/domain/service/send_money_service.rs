use crate::{
    account::application::{
        domain::model::account::AccountId,
        port_in::{
            send_money_command::SendMoneyCommand,
            send_money_use_case::SendMoneyUseCase,
        },
        port_out::{
            load_account_port::LoadAccountPort,
            update_account_state_port::UpdateAccountStatePort,
        },
    },
    common::result::AppResult,
    inject,
};

pub struct SendMoneyService {
    load_account_port: inject!(LoadAccountPort),
    update_account_port: inject!(UpdateAccountStatePort),
}

impl SendMoneyUseCase for SendMoneyService {
    fn send_money(&self, command: SendMoneyCommand) -> AppResult<()> {
        check_account_exists(command.source_account_id())?;
        check_account_exists(command.target_account_id())?;

        Ok(())
    }
}

pub fn check_account_exists(account_id: AccountId) -> AppResult<()> {
    // TODO: Implement this correctly
    Ok(())
}
