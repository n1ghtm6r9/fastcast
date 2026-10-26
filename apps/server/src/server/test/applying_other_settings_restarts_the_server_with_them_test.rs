use crate::server::test::{next_event, settings_named, start_fake_control, started, stopped};

#[tokio::test]
async fn applying_other_settings_restarts_the_server_with_them() {
    let (control, mut events) = start_fake_control("first");

    assert_eq!(next_event(&mut events).await, started("first"));

    control.apply(settings_named("second"));

    assert_eq!(next_event(&mut events).await, stopped("first"));
    assert_eq!(next_event(&mut events).await, started("second"));

    control.stop().await;
}
