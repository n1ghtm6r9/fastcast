use crate::capture::{NativeCapture, NativeError, publish_encoded_frame};
use std::time::Instant;

impl NativeCapture<'_> {
    pub(crate) fn encode_and_publish(&mut self, at: Instant) -> Result<(), NativeError> {
        let force = self.hub.take_key_frame_request();
        let encoded = self
            .encoder
            .encode(self.converter.output(), force)
            .map_err(NativeError::Encoder)?;

        if self.stopped() {
            return Ok(());
        }

        let timestamp_ms = at.saturating_duration_since(self.started).as_millis() as u32;

        publish_encoded_frame(
            self.hub,
            self.codec,
            self.fps,
            encoded.keyframe,
            &encoded.data,
            timestamp_ms,
        )?;
        self.clock.encoded(at);
        self.frames += 1;

        Ok(())
    }
}
