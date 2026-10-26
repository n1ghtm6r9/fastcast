use crate::capture::monitor_fps_for;
use crate::settings::FrameRate;

#[test]
fn a_frame_rate_choice_caps_the_monitor_capture() {
    assert_eq!(monitor_fps_for(FrameRate::Fps30, Some(144)), 30);
}
