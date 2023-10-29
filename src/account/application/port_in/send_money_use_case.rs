use super::send_money_command::SendMoneyCommand;
use crate::common::AppResult;

// What does use-case do?
//  1. Take the input.
//  2. Validate the business rules.
//  3. Manipulate the model state.
//  4. Return the output.
pub trait SendMoneyUseCase {
    fn send_money(&self, command: SendMoneyCommand) -> AppResult<()>;
}
