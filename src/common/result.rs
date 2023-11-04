use thiserror::Error as ThisError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("database disconnected")]
    DatabaseDisconnected,
}
