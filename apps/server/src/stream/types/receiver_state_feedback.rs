use crate::stream::{ReceiverState, sent_line};
use std::net::SocketAddr;
use std::time::Instant;
use tracing::info;
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

        if let Some(line) = entry.link.record(feedback, entry.fec.percent(), now) {
            let sent = sent_line(&mut entry.control.take_sent());

            info!("viewer {peer} link: {line}{sent}");
        }
    }
}
