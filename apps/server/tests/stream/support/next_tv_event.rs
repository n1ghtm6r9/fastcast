use crate::support::ARRIVAL_LIMIT;
use fastcast_native::{Link, LinkEvent};

pub fn next_tv_event(link: &Link) -> LinkEvent {
    link.next_event(ARRIVAL_LIMIT)
        .expect("an event reaches the tv link in time")
}
