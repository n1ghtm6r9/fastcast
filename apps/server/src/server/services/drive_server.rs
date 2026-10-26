use crate::server::{ServerStatus, wait_for_other_settings};
use crate::settings::Settings;
use crate::stream::StreamError;
use std::pin::pin;
use tokio::select;
use tokio::sync::{oneshot, watch};
use tracing::{error, info};

pub async fn drive_server<R, F>(
    mut settings: watch::Receiver<Settings>,
    status: watch::Sender<ServerStatus>,
    mut run: R,
) where
    R: FnMut(Settings, oneshot::Receiver<()>, watch::Sender<ServerStatus>) -> F,
    F: Future<Output = Result<(), StreamError>>,
{
    loop {
        let current = settings.borrow_and_update().clone();
        let (stop, stopped) = oneshot::channel();

        status.send_replace(ServerStatus::default());

        let mut serving = pin!(run(current.clone(), stopped, status.clone()));
        let open = select! {
            ended = &mut serving => {
                if let Err(failure) = &ended {
                    error!("the server stopped: {failure}");
                }
                status.send_replace(ServerStatus::ended(&ended));
                settings.changed().await.is_ok()
            }
            open = wait_for_other_settings(&mut settings, &current) => {
                let _ = stop.send(());
                let _ = serving.await;
                open
            }
        };

        if !open {
            status.send_replace(ServerStatus::ended(&Ok(())));

            return;
        }

        info!("starting the server with the applied settings");
    }
}
