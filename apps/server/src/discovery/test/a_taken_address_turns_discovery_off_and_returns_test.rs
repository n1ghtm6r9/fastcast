use crate::discovery::run_discovery;
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[tokio::test]
async fn a_taken_address_turns_discovery_off_and_returns() {
    let blocker = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .expect("a free port exists");
    let address = blocker.local_addr().expect("a bound socket has an address");

    let outcome = timeout(
        Duration::from_secs(5),
        run_discovery(address, "KOMPUTER".to_owned(), 3278),
    )
    .await;

    assert!(
        outcome.is_ok(),
        "discovery must give up instead of running without a socket"
    );
}
