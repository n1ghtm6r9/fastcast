use crate::capture::{NativeError, SwitchingBackend};
use crate::hub::ViewerRequest;
use tracing::warn;

impl<N, F> SwitchingBackend<N, F> {
    pub(crate) fn note_native_failure(&self, request: &ViewerRequest, cause: &NativeError) {
        if cause.is_lasting() {
            self.remember_failure_for(request);

            warn!(
                "native capture cannot work for this picture ({cause}), using ffmpeg for it until the server restarts"
            );
        } else {
            warn!(
                "native capture failed before its first frame ({cause}), using ffmpeg this time and trying native again next time"
            );
        }
    }
}
