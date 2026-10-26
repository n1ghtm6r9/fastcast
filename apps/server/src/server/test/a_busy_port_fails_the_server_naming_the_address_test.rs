use crate::server::test::{STATUS_LIMIT, settings_named};
use crate::server::{ServerStatus, run_server};
use crate::stream::StreamError;
use std::net::UdpSocket;
use tokio::sync::{oneshot, watch};
use tokio::time::timeout;

#[tokio::test]
async fn a_busy_port_fails_the_server_naming_the_address() {
    let taken = UdpSocket::bind("0.0.0.0:0").expect("a free port is bound");
    let port = taken.local_addr().expect("the port is known").port();
    let (status, _watched) = watch::channel(ServerStatus::default());
    let (_stop, stopped) = oneshot::channel();

    let ended = timeout(
        STATUS_LIMIT,
        run_server(settings_named("fastcast"), port, stopped, status),
    )
    .await
    .expect("the server gives up in time");

    assert!(matches!(ended, Err(StreamError::Bind { address, .. }) if address.port() == port));
}
