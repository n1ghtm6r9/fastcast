use crate::link::LinkEvent;
use crate::link::test::{TEST_WAIT, connected_link, patterned_bytes};
use wire::FrameKind;

#[test]
fn a_lost_shard_is_rebuilt_on_the_tv() {
    let (mut server, link, peer) = connected_link();
    let payload = patterned_bytes(20_000);
    let datagrams = server.packetize_frame(FrameKind::Video, true, &payload, 20);

    server.send_datagrams(peer, &datagrams[1..]);

    assert_eq!(link.next_event(TEST_WAIT), Some(LinkEvent::Video(payload)));
}
