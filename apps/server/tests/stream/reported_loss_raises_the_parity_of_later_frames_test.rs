use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, next_video_shard_header,
    random_payload, start_endpoint, wait_for_viewers,
};
use std::thread::sleep;
use std::time::Duration;
use wire::{Feedback, Packet, SESSION_TIMEOUT};

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

    sleep(Duration::from_millis(200));
    hub.publish(true, 1, &random_payload(3, 50_000));

    let header = next_video_shard_header(&viewer);

    assert_eq!(header.data_shards, 37);
    assert_eq!(header.parity_shards, 12);
}
