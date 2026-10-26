use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use crate::stream::{DISPLAY_RATE_RANGE, DISPLAY_SIDE_RANGE, codecs_of};
use wire::Hello;

pub fn viewer_request_of(hello: &Hello, default_fps: u32) -> ViewerRequest {
    let width = u32::from(hello.width);
    let height = u32::from(hello.height);
    let refresh_hz = if hello.fps == 0 {
        default_fps
    } else {
        u32::from(hello.fps)
    };
    let mode = (DISPLAY_SIDE_RANGE.contains(&width)
        && DISPLAY_SIDE_RANGE.contains(&height)
        && DISPLAY_RATE_RANGE.contains(&refresh_hz))
    .then_some(DisplayMode {
        width,
        height,
        refresh_hz,
    });

    ViewerRequest {
        mode,
        codecs: codecs_of(&hello.codecs),
    }
}
