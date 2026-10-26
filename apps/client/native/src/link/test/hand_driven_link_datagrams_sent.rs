use crate::link::test::HandDrivenLink;
use std::sync::atomic::Ordering;

impl HandDrivenLink {
    pub(crate) fn datagrams_sent(&self) -> u32 {
        self.shared.next_sequence.load(Ordering::Relaxed)
    }
}
