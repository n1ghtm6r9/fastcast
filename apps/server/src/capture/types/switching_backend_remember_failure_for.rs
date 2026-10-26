use crate::capture::SwitchingBackend;
use crate::hub::ViewerRequest;

impl<N, F> SwitchingBackend<N, F> {
    pub(crate) fn remember_failure_for(&self, request: &ViewerRequest) {
        *self.lock_failure() = Some(request.clone());
    }
}
