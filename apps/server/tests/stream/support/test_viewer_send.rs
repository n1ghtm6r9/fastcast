use crate::support::{TestViewer, encoded_datagram};
use wire::Packet;

impl TestViewer {
    pub fn send(&mut self, packet: &Packet<'_>) {
        let bytes = encoded_datagram(self.session, self.next_sequence, packet);

        self.next_sequence = self.next_sequence.wrapping_add(1);
        self.socket
            .send_to(&bytes, self.server)
            .expect("the packet is sent");
    }
}
