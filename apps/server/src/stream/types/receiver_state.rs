use crate::hub::Hub;
use crate::stream::{EndpointOptions, SessionEntry};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct ReceiverState {
    pub(crate) socket: Arc<UdpSocket>,
    pub(crate) hub: Arc<Hub>,
    pub(crate) options: EndpointOptions,
    pub(crate) reply: String,
    pub(crate) sessions: HashMap<SocketAddr, SessionEntry>,
    pub(crate) retired: Vec<JoinHandle<()>>,
}
