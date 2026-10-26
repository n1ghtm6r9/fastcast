use crate::server::{ServerControl, ServerStatus, drive_server};
use crate::settings::Settings;
use crate::stream::StreamError;
use tokio::runtime::Handle;
use tokio::sync::{oneshot, watch};

impl ServerControl {
    pub fn start_with<R, F>(runtime: &Handle, settings: Settings, run: R) -> ServerControl
    where
        R: FnMut(Settings, oneshot::Receiver<()>, watch::Sender<ServerStatus>) -> F
            + Send
            + 'static,
        F: Future<Output = Result<(), StreamError>> + Send + 'static,
    {
        let (settings, applied) = watch::channel(settings);
        let (status, watched) = watch::channel(ServerStatus::default());
        let driver = runtime.spawn(drive_server(applied, status, run));

        ServerControl {
            settings,
            status: watched,
            driver,
        }
    }
}
