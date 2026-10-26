use crate::link::{LinkEvent, LinkShared};

pub(crate) fn end_link(shared: &LinkShared, reason: &str) {
    shared.queue.push(LinkEvent::Ended(reason.to_string()));
    shared.queue.close();
}
