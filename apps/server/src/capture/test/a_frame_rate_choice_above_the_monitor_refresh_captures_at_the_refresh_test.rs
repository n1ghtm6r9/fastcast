use crate::capture::monitor_fps_for;
use crate::settings::FrameRate;

#[test]
fn a_frame_rate_choice_above_the_monitor_refresh_captures_at_the_refresh() {
    assert_eq!(monitor_fps_for(FrameRate::Fps240, Some(144)), 144);
}
