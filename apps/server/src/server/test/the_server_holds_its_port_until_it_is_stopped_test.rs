use crate::server::test::{STATUS_LIMIT, settings_named, status_where};
use crate::server::{ServerState, ServerStatus, run_server};
use crate::stream::stream_address;
use std::net::UdpSocket;
use tokio::sync::{oneshot, watch};
use tokio::time::timeout;

#[tokio::test]
async fn the_server_holds_its_port_until_it_is_stopped() {
    let (status, mut watched) = watch::channel(ServerStatus::default());
    let (stop, stopped) = oneshot::channel();
    let server = tokio::spawn(run_server(settings_named("fastcast"), 0, stopped, status));

    let running = status_where(&mut watched, |status| status.state != ServerState::Starting).await;
    let ServerState::Running { address } = running.state else {
        panic!("the server is not running: {:?}", running.state);
    };

    assert!(UdpSocket::bind(stream_address(address.port())).is_err());

    stop.send(()).expect("the server waits for the stop");
    timeout(STATUS_LIMIT, server)
        .await
        .expect("the server stops in time")
        .expect("the server does not panic")
        .expect("the server stops cleanly");

    assert!(UdpSocket::bind(stream_address(address.port())).is_ok());
}
