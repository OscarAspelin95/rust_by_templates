use std::{future::Future, time::Duration};
use tokio;
mod errors;
use errors::AppError;
use log::{error, info};
use simple_logger::SimpleLogger;

/// Motivation:
///
/// We want to run a future multiple times, but we can't pass a future directly
/// because it is consumed once awaited (preventing retries in the loop).
///
/// Instead, we provide a closure of type `FnMut() -> Fut` where `Fut = Future<Output = T>`.
/// FnMut() ensures a closure that can be called multiple times and that returns a future once called.
///
/// Hence `let fut_factory = || async {...}` is of type FnMut() that once called (e.g., fut_factory()), returns `async {...}` which is a future.
///
/// We need
async fn try_with_timeout<T, F, Fut>(
    mut fut_factory: F,
    timeout: u64,
    max_retries: usize,
) -> Result<T, AppError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = T>,
{
    let mut num_retries: usize = 0;

    while num_retries < max_retries {
        match tokio::time::timeout(Duration::from_secs(timeout), fut_factory()).await {
            Ok(result) => {
                info!("Suceeded after {} retries", num_retries);
                return Ok(result);
            }
            Err(_) => {
                num_retries += 1;
                info!("Retrying {}/{num_retries}", num_retries);
                continue;
            }
        }
    }

    return Err(AppError::NumRetryExceeded);
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize logger");

    match try_with_timeout(
        || async {
            tokio::time::sleep(Duration::from_secs(2)).await;
            "This is an output".to_string()
        },
        1,
        3,
    )
    .await
    {
        Ok(result) => info!("Got result {:?}", result),
        Err(e) => error!("{e}"),
    };
}
