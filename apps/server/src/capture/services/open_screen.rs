use crate::capture::{CaptureTarget, Screen, capture_fps_for, open_virtual_display};
use crate::hub::ViewerRequest;
use crate::settings::Settings;

pub async fn open_screen(settings: &Settings, request: &ViewerRequest) -> Screen {
    let display = match request.mode {
        Some(mode) if settings.virtual_display => open_virtual_display(mode).await,
        _ => None,
    };
    let target = match &display {
        Some(display) => CaptureTarget {
            output: display.output_index(),
            fps: capture_fps_for(request.mode.unwrap_or(display.mode()), display.mode()),
        },
        None => CaptureTarget {
            output: settings.capture_output,
            fps: settings.capture_fps,
        },
    };

    Screen {
        request: request.clone(),
        target,
        display,
    }
}
