use crate::capture::capture_fps_for;
use crate::display::DisplayMode;
use crate::settings::FrameRate;

#[test]
fn a_frame_rate_choice_above_the_display_rate_keeps_the_display_rate() {
    let mode = DisplayMode {
        width: 3840,
        height: 2160,
        refresh_hz: 120,
    };

    assert_eq!(capture_fps_for(mode, mode, FrameRate::Fps240), 120);
}
