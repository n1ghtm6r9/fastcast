use super::start_responder;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[tokio::test]
async fn aborting_the_responder_releases_its_udp_port() {
    let mut responder = start_responder("KOMPUTER", 3278).await;

    responder.task.abort();

    let outcome = timeout(Duration::from_secs(5), &mut responder.task)
        .await
        .expect("the responder stops in time");

    assert!(
        outcome
            .expect_err("an aborted responder does not finish normally")
            .is_cancelled()
    );
    UdpSocket::bind(responder.address)
        .await
        .expect("the port is free again");
}
