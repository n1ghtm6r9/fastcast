use crate::support::{
    hub_with_config, second_viewer_is_served_after_the_first_vanished, start_endpoint_on,
};
use std::net::{Ipv4Addr, UdpSocket};
use wire::SESSION_TIMEOUT;

#[test]
fn the_endpoint_survives_a_socket_that_reports_connection_resets() {
    let hub = hub_with_config();
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists");
    let endpoint = start_endpoint_on(socket, hub.clone(), SESSION_TIMEOUT);

    second_viewer_is_served_after_the_first_vanished(&endpoint, &hub);
}
