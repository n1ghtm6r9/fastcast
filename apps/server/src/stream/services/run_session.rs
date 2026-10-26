use crate::hub::{Hub, Subscription, ViewerRequest};
use crate::stream::{
    HubEvent, MAX_REPLAY_FRAMES, SessionControl, SessionCursor, SessionSender, deliver_frame,
    next_hub_event,
};
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use tokio::runtime::Builder;
use tracing::warn;

pub fn run_session(
    socket: Arc<UdpSocket>,
    peer: SocketAddr,
    session: u32,
    nonce: u32,
    hub: Arc<Hub>,
    request: ViewerRequest,
    control: Arc<SessionControl>,
) {
    let runtime = match Builder::new_current_thread().build() {
        Ok(runtime) => runtime,
        Err(error) => {
            warn!(%peer, "could not start the runtime of a stream session: {error}");

            return;
        }
    };

    let mut sender = SessionSender::new(socket, peer, session, control.clone());

    sender.send_welcome(nonce);
    hub.request(request);

    let Subscription {
        config,
        gop,
        mut frames,
        guard,
    } = hub.join();
    let mut cursor = SessionCursor::starting_with(config.clone());

    if let Some(config) = &config {
        sender.send_config(config);
    }

    if (1..=MAX_REPLAY_FRAMES).contains(&gop.len()) {
        for frame in &gop {
            deliver_frame(&mut sender, &mut cursor, frame);
        }
    } else {
        hub.request_key_frame();
    }

    while !control.is_stopped() {
        if control.take_welcome() {
            sender.send_welcome(nonce);
        }

        if control.take_key_frame_need(sender.last_key_frame()) && !cursor.is_waiting() {
            cursor.resync();
            hub.request_key_frame();
        }

        match runtime.block_on(next_hub_event(&mut frames, &control)) {
            HubEvent::Frame(frame) => deliver_frame(&mut sender, &mut cursor, &frame),
            HubEvent::Lagged => {
                cursor.lagged();
                hub.request_key_frame();
            }
            HubEvent::Woken => {}
            HubEvent::Closed => break,
        }
    }

    sender.send_bye();
    drop(guard);
}
