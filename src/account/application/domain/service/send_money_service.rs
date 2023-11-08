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
    use crate::account::application::{
        domain::{
            model::{
                account::{Account, AccountId},
                activity_window::ActivityWindow,
                money::Money,
            },
            service::send_money_service::SendMoneyService,
        },
        port_in::{
            send_money_command::SendMoneyCommand,
            send_money_use_case::SendMoneyUseCase,
        },
        port_out::{
            load_account_port::MockLoadAccountPort,
            update_account_state_port::MockUpdateAccountStatePort,
        },
    };

    #[test]
    fn transaction_succeeds() {
        // Arrange
        let source_account = Account {
            id: AccountId(1),
            baseline_balance: Money(1000),
            activity_window: ActivityWindow::default(),
        };
        let target_account = Account {
            id: AccountId(2),
            baseline_balance: Money(1000),
            activity_window: ActivityWindow::default(),
        };
        let service = SendMoneyService {
            load_account_port: Box::new(MockLoadAccountPort::new()),
            update_account_port: Box::new(MockUpdateAccountStatePort::new()),
        };
        let money = Money(500);
        let command =
            SendMoneyCommand::new(source_account.id, target_account.id, money)
                .unwrap();

        // Act
        let result = service.send_money(command);

        // Assert
        assert!(result.is_ok());
        assert_eq!(source_account.calculate_balance(), Money(500));
        assert_eq!(target_account.calculate_balance(), Money(1500));
    }
}
