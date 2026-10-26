use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, next_video_shard_header_after,
    random_payload, start_endpoint, wait_for_viewers, wait_until,
};
use wire::{Feedback, Packet, SESSION_TIMEOUT, ShardHeader};

#[test]
fn reported_loss_raises_the_parity_of_later_frames() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut viewer);
    wait_for_viewers(&hub, 1);

    viewer.send(&Packet::Feedback(Feedback {
        received_packets: 100,
        highest_sequence: 99,
        ..Feedback::default()
    }));
    viewer.send(&Packet::Feedback(Feedback {
        received_packets: 190,
        highest_sequence: 199,
        ..Feedback::default()
    }));

    let payload = random_payload(3, 50_000);
    let mut header: Option<ShardHeader> = None;
    let mut timestamp = 0;

    wait_until(SESSION_TIMEOUT / 2, || {
        timestamp += 1;
        hub.publish(true, timestamp, &payload);

        let newest = header.map(|seen| seen.frame);
        let seen = next_video_shard_header_after(&viewer, newest);

        header = Some(seen);

        seen.parity_shards == 12
    });

    let header = header.expect("a video frame arrives in time");

    assert_eq!(header.data_shards, 37);
    assert_eq!(header.parity_shards, 12);
}
