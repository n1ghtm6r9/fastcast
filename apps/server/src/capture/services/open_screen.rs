use crate::capture::{CaptureTarget, Screen, codecs_or_avc, open_virtual_display};
use crate::hub::ViewerRequest;
use crate::settings::Settings;

pub async fn open_screen(settings: &Settings, request: &ViewerRequest) -> Screen {
    let codecs = codecs_or_avc(&request.codecs);
    let display = match request.mode {
        Some(mode) if settings.virtual_display => open_virtual_display(mode).await,
        _ => None,
    };
    let target = match &display {
        Some(display) => CaptureTarget {
            output: display.output_index(),
            fps: display.mode().refresh_hz,
            codecs,
        },
        None => CaptureTarget {
            output: settings.capture_output,
            fps: settings.capture_fps,
            codecs,
        },
    };

    Screen {
        request: request.clone(),
        target,
        display,
    }
}
