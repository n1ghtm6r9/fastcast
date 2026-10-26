use crate::link::{EventQueue, LinkEvent, MAX_QUEUED_FRAMES};
use wire::{FrameKind, ReceivedFrame};

impl EventQueue {
    pub fn push_frame(&self, frame: ReceivedFrame) {
        let key_video = frame.kind == FrameKind::Video && frame.keyframe;
        let config = frame.kind == FrameKind::Config;
        let mut state = self.lock();

        if state.waiting_for_key_frame && !key_video && !config {
            return;
        }

        if state.frames >= MAX_QUEUED_FRAMES {
            state.drop_frames_except_the_latest_config();
            state.waiting_for_key_frame = true;

            if !key_video && !config {
                return;
            }
        }

        if key_video {
            state.waiting_for_key_frame = false;
        }

        let event = if config {
            LinkEvent::Config(frame.payload)
        } else {
            LinkEvent::Video(frame.payload)
        };

        state.frames += 1;
        state.events.push_back(event);
        drop(state);
        self.ready.notify_one();
    }
}
