use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use crate::server::ServerStatus;
use crate::window::test::running_status;

pub fn watching_at(refresh_hz: u32) -> ServerStatus {
    ServerStatus {
        viewers: 1,
        request: ViewerRequest {
            mode: Some(DisplayMode {
                width: 3840,
                height: 2160,
                refresh_hz,
            }),
            ..ViewerRequest::default()
        },
        ..running_status()
    }
}
