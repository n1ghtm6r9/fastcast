use crate::web::bind_listener;

#[tokio::test]
async fn binding_port_zero_picks_a_free_port() {
    let listener = bind_listener("127.0.0.1", 0)
        .await
        .expect("a free port exists");
    let address = listener
        .local_addr()
        .expect("a bound listener has an address");

    assert_ne!(address.port(), 0);
    assert!(address.ip().is_loopback());
}
