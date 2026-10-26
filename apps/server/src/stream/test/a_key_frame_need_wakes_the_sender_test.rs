use super::{feedback_of, woken_within_two_seconds};
use crate::stream::SessionControl;

#[tokio::test]
async fn a_key_frame_need_wakes_the_sender() {
    let control = SessionControl::new();

    control.report(&feedback_of(0, true), 10);

    assert!(woken_within_two_seconds(&control).await);
    assert!(!control.is_stopped());
}
