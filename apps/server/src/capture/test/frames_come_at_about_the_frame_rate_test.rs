use super::frames_in_the_second_after_the_first;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn frames_come_at_about_the_frame_rate() {
    let frames = frames_in_the_second_after_the_first(60).await;

    assert!(
        (59..=61).contains(&frames),
        "{frames} frames in one second at 60 fps"
    );
}
