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
    pub load_account_port: inject!(LoadAccountPort),
    pub update_account_port: inject!(UpdateAccountStatePort),
}

impl SendMoneyUseCase for SendMoneyService {
    /// [[Transactional]]
    /// Send money to other account.
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

// #[cfg(test)]
// mod test {
//     use crate::account::application::domain::service::send_money_service::SendMoneyService;
//
//     #[test]
//     fn transaction_succeeds() {
//         // Arrange
//         let service = SendMoneyService{load_account_port: }
//
//
//         // Act
//         let result = service.send_money(command);
//
//         // Assert
//         assert!(result.is_ok());
//         assert!()
//     }
// }
