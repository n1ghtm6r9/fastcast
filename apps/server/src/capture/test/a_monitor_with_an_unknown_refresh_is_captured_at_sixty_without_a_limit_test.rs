use crate::capture::monitor_fps_for;
use crate::settings::FrameRate;

#[test]
fn a_monitor_with_an_unknown_refresh_is_captured_at_sixty_without_a_limit() {
    assert_eq!(monitor_fps_for(FrameRate::Unlimited, None), 60);
}
