use crate::capture::monitor_fps_for;
use crate::settings::FrameRate;

#[test]
fn a_monitor_with_an_unknown_refresh_is_captured_at_the_chosen_frame_rate() {
    assert_eq!(monitor_fps_for(FrameRate::Fps120, None), 120);
}
