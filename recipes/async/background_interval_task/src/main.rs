use log::info;
use simple_logger::SimpleLogger;
use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedSender, error::TryRecvError};

async fn run_background_interval_task(tick: u64) -> UnboundedSender<String> {
    let (tx, mut rc) = mpsc::unbounded_channel::<String>();
    let mut interval = tokio::time::interval(Duration::from_secs(tick));

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            match rc.try_recv() {
                Ok(msg) => info!("[background_task] got message `{}`!", msg),
                Err(e) => match e {
                    TryRecvError::Empty => info!("[background_task] no new message :("),
                    TryRecvError::Disconnected => {
                        info!("[background_task] disconnected");
                        break;
                    }
                },
            };
        }
    });

    tx
}

fn simulate_work() {
    std::thread::sleep(Duration::from_secs(2));
    info!("[main] Doing some work in main...");
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize logger");

    let tx = run_background_interval_task(1).await;
    info!("[main] Triggered background task");

    simulate_work();
    let _ = tx.send("Hello from main!".to_string());

    simulate_work();
    drop(tx);
    simulate_work();
}
