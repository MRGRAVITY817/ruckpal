use crate::{
    account::application::domain::model::account::Account,
    common::result::AppResult,
};

pub trait UpdateAccountStatePort {
    fn update_activities(&self, account: Account) -> AppResult<()>;
}
