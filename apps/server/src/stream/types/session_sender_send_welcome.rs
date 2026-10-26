use crate::stream::SessionSender;
use wire::{Packet, SHARD_SIZE, Welcome};

impl SessionSender {
    pub fn send_welcome(&mut self, nonce: u32) {
        self.send_control(&Packet::Welcome(Welcome {
            nonce,
            shard_size: SHARD_SIZE as u16,
        }));
    }
}
