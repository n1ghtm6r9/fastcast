use crate::stream::ReceiverState;
use std::net::SocketAddr;
use std::time::Instant;
use wire::Feedback;

impl ReceiverState {
    pub(crate) fn feedback(
        &mut self,
        peer: SocketAddr,
        session: u32,
        feedback: &Feedback,
        now: Instant,
    ) {
        let Some(entry) = self
            .sessions
            .get_mut(&peer)
            .filter(|entry| entry.id == session)
        else {
            self.refuse_stranger(peer, session);

            return;
        };

        entry.last_seen = now;

        if let Some(loss) = entry.loss.update(feedback) {
            entry.fec.observe(loss, now);
        }

        entry.control.report(feedback, entry.fec.percent());
    }
}
