use crate::{
    account::application::domain::model::account::AccountId,
    common::result::AppResult,
};

use super::account_entity::AccountEntity;

pub trait AccountRepository {
    fn find_by_id(&self, id: AccountId) -> AppResult<AccountEntity>;
}
