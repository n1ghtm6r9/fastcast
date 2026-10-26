use crate::stream::{SessionSender, frame_interval_of, send_paced};
use wire::OutgoingFrame;

impl SessionSender {
    pub(crate) fn send_frame(&mut self, outgoing: &OutgoingFrame<'_>, fps: u32) -> u32 {
        let number =
            self.packetizer
                .frame(outgoing, self.control.fec_percent(), &mut self.datagrams);

        send_paced(
            &self.socket,
            self.peer,
            &self.datagrams,
            frame_interval_of(fps),
        );

        number
    }
}
