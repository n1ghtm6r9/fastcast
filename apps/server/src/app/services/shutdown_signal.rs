use tracing::{info, warn};

pub async fn shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => info!("Ctrl+C received, shutting down"),
        Err(error) => {
            warn!("cannot listen for Ctrl+C: {error}");
            std::future::pending::<()>().await;
        }
    }
}
