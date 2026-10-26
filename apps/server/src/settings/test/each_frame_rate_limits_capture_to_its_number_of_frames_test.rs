use crate::settings::FrameRate;

#[test]
fn each_frame_rate_limits_capture_to_its_number_of_frames() {
    let limits = [
        FrameRate::Fps24,
        FrameRate::Fps30,
        FrameRate::Fps60,
        FrameRate::Fps120,
        FrameRate::Fps240,
        FrameRate::Unlimited,
    ]
    .map(FrameRate::limit);

    assert_eq!(
        limits,
        [Some(24), Some(30), Some(60), Some(120), Some(240), None]
    );
}
