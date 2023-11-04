use super::{
    account_repository::AccountRepository,
    activity_repository::ActivityRepository,
};
use crate::{
    account::application::{
        domain::model::account::{Account, AccountId},
        port_out::{
            load_account_port::LoadAccountPort,
            update_account_state_port::UpdateAccountStatePort,
        },
    },
    common::{result::AppResult, timestamp::Timestamp},
    inject,
};

pub struct AccountPersistenceAdapter {
    account_repo: inject!(AccountRepository),
    activity_repo: inject!(ActivityRepository),
}

impl LoadAccountPort for AccountPersistenceAdapter {
    fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: Timestamp,
    ) -> AppResult<Account> {
        let account = self.account_repo.find_by_id(account_id)?;

        let activities = self
            .activity_repo
            .find_by_owner_since(account_id, baseline_date)?;

        let withdrawal_balance = self
            .activity_repo
            .get_withdrawal_balance_until(account_id, baseline_date)
            .or(Ok(0))?;

        let deposit_balance = self
            .activity_repo
            .get_deposit_balance_until(account_id, baseline_date)
            .or(Ok(0))?;

        Ok(account.into_account(
            deposit_balance,
            withdrawal_balance,
            activities,
        ))
    }
}

impl UpdateAccountStatePort for AccountPersistenceAdapter {
    fn update_activities(&self, account: Account) -> AppResult<()> {
        let activities = account
            .activity_window
            .get_activities()
            .to_vec()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        self.activity_repo.save_many(activities)
    }
}
