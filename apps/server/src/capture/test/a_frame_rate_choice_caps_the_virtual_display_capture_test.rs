use crate::capture::capture_fps_for;
use crate::display::DisplayMode;
use crate::settings::FrameRate;

#[test]
fn a_frame_rate_choice_caps_the_virtual_display_capture() {
    let mode = DisplayMode {
        width: 3840,
        height: 2160,
        refresh_hz: 120,
    };

    assert_eq!(capture_fps_for(mode, mode, FrameRate::Fps60), 60);
}
