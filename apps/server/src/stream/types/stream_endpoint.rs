use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;

pub struct StreamEndpoint {
    pub(crate) local: SocketAddr,
    pub(crate) stop: Arc<AtomicBool>,
    pub(crate) receiver: Option<JoinHandle<()>>,
}
