use crate::stream::SessionControl;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use wire::{Datagrams, Packetizer};

pub struct SessionSender {
    pub(crate) socket: Arc<UdpSocket>,
    pub(crate) peer: SocketAddr,
    pub(crate) control: Arc<SessionControl>,
    pub(crate) packetizer: Packetizer,
    pub(crate) datagrams: Datagrams,
    pub(crate) scratch: Vec<u8>,
    pub(crate) last_key_frame: Option<u32>,
}

impl SessionSender {
    pub fn new(
        socket: Arc<UdpSocket>,
        peer: SocketAddr,
        session: u32,
        control: Arc<SessionControl>,
    ) -> Self {
        Self {
            socket,
            peer,
            control,
            packetizer: Packetizer::new(session),
            datagrams: Datagrams::new(),
            scratch: Vec::new(),
            last_key_frame: None,
        }
    }
}
