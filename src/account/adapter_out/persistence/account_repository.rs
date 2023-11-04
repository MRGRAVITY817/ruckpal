use super::account_persistence_adapter::AccountEntity;
use crate::{
    account::application::domain::model::account::AccountId,
    common::result::AppResult,
};

pub trait AccountRepository {
    fn find_by_id(id: AccountId) -> AppResult<AccountEntity>;
}
