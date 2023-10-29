use crate::{
    account::application::{
        port_in::{
            send_money_command::SendMoneyCommand,
            send_money_use_case::SendMoneyUseCase,
        },
        port_out::{
            load_account_port::LoadAccountPort,
            update_account_state_port::UpdateAccountStatePort,
        },
    },
    common::AppResult,
};

pub struct SendMoneyService {
    load_account_port: Box<dyn LoadAccountPort>,
    update_account_port: Box<dyn UpdateAccountStatePort>,
}

impl SendMoneyUseCase for SendMoneyService {
    fn send_money(&self, command: SendMoneyCommand) -> AppResult<()> {
        Ok(())
    }
}
