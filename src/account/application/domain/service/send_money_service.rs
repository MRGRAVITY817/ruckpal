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
    common::result::AppResult,
    impl_t,
};

pub struct SendMoneyService {
    load_account_port: impl_t!(LoadAccountPort),
    update_account_port: impl_t!(UpdateAccountStatePort),
}

impl SendMoneyUseCase for SendMoneyService {
    fn send_money(&self, command: SendMoneyCommand) -> AppResult<()> {
        Ok(())
    }
}
