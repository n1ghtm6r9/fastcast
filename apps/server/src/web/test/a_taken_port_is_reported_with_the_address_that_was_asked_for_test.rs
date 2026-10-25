use crate::web::{WebError, bind_listener};

#[tokio::test]
async fn a_taken_port_is_reported_with_the_address_that_was_asked_for() {
    let first = bind_listener("127.0.0.1", 0)
        .await
        .expect("a free port exists");
    let port = first
        .local_addr()
        .expect("a bound listener has an address")
        .port();

    let failure = bind_listener("127.0.0.1", port)
        .await
        .expect_err("the port is taken");

    assert!(matches!(
        &failure,
        WebError::Bind { address, .. } if address == &format!("127.0.0.1:{port}")
    ));
    assert!(
        failure
            .to_string()
            .contains("is another instance already running?")
    );
}
