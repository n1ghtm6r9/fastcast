use super::woken_within_two_seconds;
use crate::stream::SessionControl;

#[tokio::test]
async fn a_stopped_control_wakes_its_sender() {
    let control = SessionControl::new();

    control.stop();

    assert!(woken_within_two_seconds(&control).await);
    assert!(control.is_stopped());
}
