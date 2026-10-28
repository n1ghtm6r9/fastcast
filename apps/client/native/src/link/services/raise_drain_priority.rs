use crate::link::{DRAIN_THREAD_PRIORITY, LINK_THREAD_PRIORITY, raise_thread_priority};

pub(crate) fn raise_drain_priority() {
    if !raise_thread_priority(DRAIN_THREAD_PRIORITY) {
        raise_thread_priority(LINK_THREAD_PRIORITY);
    }
}
