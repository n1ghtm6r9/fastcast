use crate::link::LinkShared;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Link {
    pub(crate) socket: Arc<UdpSocket>,
    pub(crate) server: SocketAddr,
    pub(crate) shared: Arc<LinkShared>,
    pub(crate) receive_buffer_bytes: u32,
    pub(crate) thread: Mutex<Option<JoinHandle<()>>>,
}
