use app::hub::Hub;
use app::stream::{EndpointOptions, StreamEndpoint, bind_stream_socket};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

pub fn start_endpoint(hub: Arc<Hub>, session_timeout: Duration) -> StreamEndpoint {
    let socket =
        bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists");
    let options = EndpointOptions {
        session_timeout,
        ..EndpointOptions::new("test-pc".to_owned(), 60)
    };

    StreamEndpoint::start(socket, hub, options).expect("the endpoint starts")
}
