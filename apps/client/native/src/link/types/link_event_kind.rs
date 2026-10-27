use crate::link::{
    EVENT_CONFIG, EVENT_CONNECTED, EVENT_ENDED, EVENT_NOTICE, EVENT_VIDEO, LinkEvent,
};

impl LinkEvent {
    pub fn kind(&self) -> u8 {
        match self {
            LinkEvent::Video(_) => EVENT_VIDEO,
            LinkEvent::Config(_) => EVENT_CONFIG,
            LinkEvent::Connected => EVENT_CONNECTED,
            LinkEvent::Ended(_) => EVENT_ENDED,
            LinkEvent::Notice(_) => EVENT_NOTICE,
        }
    }
}
