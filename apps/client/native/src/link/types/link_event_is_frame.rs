use crate::link::LinkEvent;

impl LinkEvent {
    pub(crate) fn is_frame(&self) -> bool {
        matches!(self, LinkEvent::Config(_) | LinkEvent::Video(_))
    }
}
