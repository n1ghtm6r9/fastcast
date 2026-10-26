use crate::support::start_endpoint_on;
use app::hub::Hub;
use app::stream::{StreamEndpoint, bind_stream_socket};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

pub fn start_endpoint(hub: Arc<Hub>, session_timeout: Duration) -> StreamEndpoint {
    let socket =
        bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists");

    start_endpoint_on(socket, hub, session_timeout)
}
