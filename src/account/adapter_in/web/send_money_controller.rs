use crate::{
    account::application::{
        domain::model::{account::AccountId, money::Money},
        port_in::{
            send_money_command::SendMoneyCommand,
            send_money_use_case::SendMoneyUseCase,
        },
    },
    inject,
};
use axum::{extract::State, Json};
use http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendMoneyPayload {
    source_account_id: AccountId,
    target_account_id: AccountId,
    amount: u32,
}

pub async fn send_money_controller(
    State(send_money_use_case): State<inject!(SendMoneyUseCase)>,
    Json(payload): Json<SendMoneyPayload>,
) -> StatusCode {
    match SendMoneyCommand::new(
        payload.source_account_id,
        payload.target_account_id,
        Money(payload.amount as i64),
    )
    .and_then(|command| send_money_use_case.send_money(command))
    {
        Ok(_) => StatusCode::OK,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
