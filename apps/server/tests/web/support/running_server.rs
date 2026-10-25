use app::web::WebError;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub struct RunningServer {
    pub address: SocketAddr,
    pub shutdown: oneshot::Sender<()>,
    pub task: JoinHandle<Result<(), WebError>>,
}
