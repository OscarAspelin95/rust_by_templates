use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid socket address")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidSocketAddress(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::IOError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
