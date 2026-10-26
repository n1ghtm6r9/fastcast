use crate::stream::SessionSender;
use tracing::debug;
use wire::Packet;

impl SessionSender {
    pub(crate) fn send_control(&mut self, packet: &Packet<'_>) {
        self.packetizer.control(packet, &mut self.scratch);

        if let Err(error) = self.socket.send_to(&self.scratch, self.peer) {
            debug!(peer = %self.peer, "a control datagram was not sent: {error}");
        }
    }
}
