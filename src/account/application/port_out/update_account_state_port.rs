use crate::account::application::domain::model::account::Account;

pub trait UpdateAccountStatePort {
    fn update_activities(&self, account: Account);
}
