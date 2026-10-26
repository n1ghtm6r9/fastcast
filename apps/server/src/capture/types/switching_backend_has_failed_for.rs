use crate::capture::SwitchingBackend;
use crate::hub::ViewerRequest;

impl<N, F> SwitchingBackend<N, F> {
    pub(crate) fn has_failed_for(&self, request: &ViewerRequest) -> bool {
        self.lock_failure().as_ref() == Some(request)
    }
}
