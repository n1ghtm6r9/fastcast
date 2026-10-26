use crate::capture::capture_fps_for;
use crate::display::DisplayMode;
use crate::settings::FrameRate;

#[test]
fn a_display_shown_slower_than_requested_is_captured_at_its_own_rate() {
    let requested = DisplayMode {
        width: 3840,
        height: 2160,
        refresh_hz: 100,
    };
    let shown = DisplayMode {
        width: 3840,
        height: 2160,
        refresh_hz: 60,
    };

    assert_eq!(capture_fps_for(requested, shown, FrameRate::Unlimited), 60);
}
