use crate::capture::{NativeCapture, NativeError, key_frame_wanted, publish_encoded_frame};
use std::time::Instant;

impl NativeCapture<'_> {
    pub(crate) fn encode_and_publish(&mut self, at: Instant) -> Result<(), NativeError> {
        let shown = self.pending_shown.take();
        let arrived = self.pending_arrived.take();
        let force = key_frame_wanted(&self.key_frames, self.hub, at);
        let encoded = self
            .encoder
            .encode(self.converter.output(), force)
            .map_err(NativeError::Encoder)?;

        if encoded.keyframe {
            self.key_frames.forced(at);
        }

        if self.stopped() {
            return Ok(());
        }

        let timestamp_ms = at.saturating_duration_since(self.started).as_millis() as u32;

        let published = publish_encoded_frame(
            self.hub,
            self.codec,
            self.fps,
            encoded.keyframe,
            &encoded.data,
            timestamp_ms,
            shown,
        )?;
        match arrived {
            Some(arrived) => self.clock.encoded_picture(arrived, at),
            None => self.clock.encoded(at),
        }

        if published {
            self.frames += 1;
        }

        Ok(())
    }
}
