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

#[cfg(test)]
mod test {
    use crate::{
        account::application::{
            domain::{
                model::{
                    account::{Account, AccountId},
                    activity_window::ActivityWindow,
                    money::Money,
                },
                service::send_money_service::SendMoneyService,
            },
            port_out::{
                load_account_port::LoadAccountPort,
                update_account_state_port::UpdateAccountStatePort,
            },
        },
        common::{result::AppResult, timestamp::Timestamp},
    };

    // #[test]
    // fn transaction_succeeds() {
    //     // Arrange
    //     let service = SendMoneyService {
    //         load_account_port: ,
    //         update_account_port: ,
    //     };
    //
    //     // Act
    //     let result = service.send_money(command);
    //
    //     // Assert
    //     assert!(result.is_ok());
    //     assert!()
    // }
    //
    struct MockLoadAccountPort;

    impl LoadAccountPort for MockLoadAccountPort {
        fn load_account(
            &self,
            account_id: AccountId,
            baseline_date: Timestamp,
        ) -> AppResult<Account> {
            Ok(Account {
                id: account_id,
                baseline_balance: Money(0),
                activity_window: ActivityWindow::default(),
            })
        }
    }

    struct MockUpdateAccountStatePort;

    impl UpdateAccountStatePort for MockUpdateAccountStatePort {
        fn update_activities(&self, account: Account) -> AppResult<()> {
            Ok(())
        }
    }
}
