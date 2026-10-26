use crate::server::test::{
    next_event, remaining_events, settings_named, start_fake_control, started, stopped,
};

#[tokio::test]
async fn applying_the_same_settings_keeps_the_server_running() {
    let (control, mut events) = start_fake_control("first");

    assert_eq!(next_event(&mut events).await, started("first"));

    control.apply(settings_named("first"));
    control.stop().await;

    assert_eq!(remaining_events(events).await, vec![stopped("first")]);
}
