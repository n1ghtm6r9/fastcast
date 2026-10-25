use std::net::SocketAddr;
use tokio::task::JoinHandle;

pub struct Responder {
    pub address: SocketAddr,
    pub task: JoinHandle<()>,
}
