use super::frames_in_the_second_after_the_first;

#[tokio::test]
#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]
async fn frames_come_at_about_the_frame_rate_at_120_fps() {
    let frames = frames_in_the_second_after_the_first(120).await;

    assert!(
        (118..=122).contains(&frames),
        "{frames} frames in one second at 120 fps"
    );
}
