use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Num retries exceeded")]
    NumRetryExceeded,
}
