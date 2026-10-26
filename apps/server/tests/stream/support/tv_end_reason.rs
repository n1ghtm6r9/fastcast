use crate::support::ARRIVAL_LIMIT;
use fastcast_native::{Link, LinkEvent};

pub fn tv_end_reason(link: &Link) -> Option<String> {
    loop {
        if let LinkEvent::Ended(reason) = link.next_event(ARRIVAL_LIMIT)? {
            return Some(reason);
        }
    }
}
