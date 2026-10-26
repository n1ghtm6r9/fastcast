use crate::server::{ServerStatus, run_server};
use crate::settings::Settings;
use crate::stream::STREAM_PORT;
use tokio::runtime::Handle;
use tokio::sync::watch;
use tokio::task::JoinHandle;

pub struct ServerControl {
    pub(crate) settings: watch::Sender<Settings>,
    pub(crate) status: watch::Receiver<ServerStatus>,
    pub(crate) driver: JoinHandle<()>,
}

impl ServerControl {
    pub fn start(runtime: &Handle, settings: Settings) -> ServerControl {
        Self::start_with(runtime, settings, |settings, stop, status| {
            run_server(settings, STREAM_PORT, stop, status)
        })
    }
}
