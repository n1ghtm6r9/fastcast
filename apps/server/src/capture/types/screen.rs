use crate::capture::CaptureTarget;
use crate::display::VirtualDisplay;
use crate::hub::ViewerRequest;

pub struct Screen {
    pub request: ViewerRequest,
    pub target: CaptureTarget,
    pub display: Option<VirtualDisplay>,
}
