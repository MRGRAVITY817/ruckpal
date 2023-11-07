use super::account_entity::AccountEntity;
use crate::{
    account::application::domain::model::account::AccountId,
    common::result::AppResult,
};

pub trait AccountRepository {
    fn find_by_id(&self, id: AccountId) -> AppResult<AccountEntity>;
}
