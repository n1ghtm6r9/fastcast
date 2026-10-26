use app::hub::Hub;
use app::stream::{EndpointOptions, StreamEndpoint};
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;

pub fn start_endpoint_on(
    socket: UdpSocket,
    hub: Arc<Hub>,
    session_timeout: Duration,
) -> StreamEndpoint {
    let options = EndpointOptions {
        session_timeout,
        ..EndpointOptions::new("test-pc".to_owned(), 60)
    };

    StreamEndpoint::start(socket, hub, options).expect("the endpoint starts")
}
