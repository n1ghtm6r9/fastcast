use crate::support::TestViewer;
use std::time::Instant;
use wire::{Packet, decode_datagram};

impl TestViewer {
    pub(crate) fn take(&mut self, bytes: &[u8]) {
        let Some(datagram) = decode_datagram(bytes) else {
            return;
        };

        if datagram.session != self.session {
            return;
        }

        self.counters.record(datagram.sequence);

        if let Packet::Shard(shard) = &datagram.packet {
            self.receiver.accept(shard, Instant::now());
        }
    }
}
