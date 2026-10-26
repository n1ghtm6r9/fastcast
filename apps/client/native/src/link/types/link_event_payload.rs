use crate::link::LinkEvent;

impl LinkEvent {
    pub fn payload(&self) -> &[u8] {
        match self {
            LinkEvent::Connected => &[],
            LinkEvent::Config(bytes) | LinkEvent::Video(bytes) => bytes,
            LinkEvent::Ended(reason) => reason.as_bytes(),
        }
    }
}
