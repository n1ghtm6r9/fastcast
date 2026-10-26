use crate::capture::monitor_fps_for;
use crate::settings::FrameRate;

#[test]
fn an_unlimited_frame_rate_captures_a_monitor_at_its_refresh_rate() {
    assert_eq!(monitor_fps_for(FrameRate::Unlimited, Some(144)), 144);
}
