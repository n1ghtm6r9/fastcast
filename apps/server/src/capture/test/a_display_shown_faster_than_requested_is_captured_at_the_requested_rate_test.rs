use crate::capture::capture_fps_for;
use crate::display::DisplayMode;

#[test]
fn a_display_shown_faster_than_requested_is_captured_at_the_requested_rate() {
    let requested = DisplayMode {
        width: 3840,
        height: 2160,
        refresh_hz: 120,
    };
    let shown = DisplayMode {
        width: 3840,
        height: 2160,
        refresh_hz: 240,
    };

    assert_eq!(capture_fps_for(requested, shown), 120);
}
