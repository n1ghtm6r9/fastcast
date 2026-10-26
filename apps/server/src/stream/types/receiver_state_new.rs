use crate::hub::Hub;
use crate::stream::{EndpointOptions, ReceiverState, STREAM_PORT, server_reply_json};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::Arc;

impl ReceiverState {
    pub fn new(socket: Arc<UdpSocket>, hub: Arc<Hub>, options: EndpointOptions) -> ReceiverState {
        let port = socket
            .local_addr()
            .map_or(STREAM_PORT, |address| address.port());
        let reply = server_reply_json(&options.server_name, port);

        ReceiverState {
            socket,
            hub,
            options,
            reply,
            sessions: HashMap::new(),
            retired: Vec::new(),
        }
    }
}
