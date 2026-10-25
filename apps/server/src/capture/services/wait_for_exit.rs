use crate::capture::EXIT_WAIT_TIMEOUT;
use std::process::ExitStatus;
use tokio::process::Child;
use tokio::time::timeout;

pub async fn wait_for_exit(child: &mut Child) -> Option<ExitStatus> {
    if let Ok(Ok(status)) = timeout(EXIT_WAIT_TIMEOUT, child.wait()).await {
        return Some(status);
    }

    let _ = child.kill().await;

    None
}
