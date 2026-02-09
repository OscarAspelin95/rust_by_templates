use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid socket address")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("IO error")]
    IOError(#[from] std::io::Error),
}
