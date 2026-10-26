use std::net::{SocketAddr, UdpSocket};
use wire::{FrameReceiver, ReceiveCounters, Welcome};

pub struct TestViewer {
    pub(crate) socket: UdpSocket,
    pub(crate) server: SocketAddr,
    pub(crate) session: u32,
    pub(crate) welcome: Welcome,
    pub(crate) next_sequence: u32,
    pub(crate) counters: ReceiveCounters,
    pub(crate) receiver: FrameReceiver,
}
