use crate::stream::SessionControl;
use std::net::UdpSocket;
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct RunningSession {
    pub peer: UdpSocket,
    pub control: Arc<SessionControl>,
    pub thread: JoinHandle<()>,
}
