use super::woken_within_two_seconds;
use crate::stream::SessionControl;

#[tokio::test]
async fn a_requested_welcome_wakes_its_sender() {
    let control = SessionControl::new();

    control.want_welcome();

    assert!(woken_within_two_seconds(&control).await);
    assert!(control.take_welcome());
}
