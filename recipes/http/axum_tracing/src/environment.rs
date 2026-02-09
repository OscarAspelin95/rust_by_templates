use crate::errors::AppError;
use std::env;
use std::net::SocketAddr;

pub struct Environment {
    port: String,
    addr: String,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            addr: env::var("ADDR").unwrap_or_else(|_| "127.0.0.1".to_string()),
        }
    }

    pub fn socket_address(&self) -> Result<SocketAddr, AppError> {
        let socket_address = format!("{}:{}", self.addr, self.port).parse::<SocketAddr>()?;
        Ok(socket_address)
    }
}
