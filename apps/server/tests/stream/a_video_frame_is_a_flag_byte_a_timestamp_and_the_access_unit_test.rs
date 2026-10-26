use crate::support::{
    ARRIVAL_LIMIT, FRAME_DATA, TestViewer, a_bare_hello, hub_with_config, next_config,
    start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn a_video_frame_is_a_flag_byte_a_timestamp_and_the_access_unit() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut viewer);
    hub.publish(true, 0x0102_0304, &FRAME_DATA);

    let frame = viewer
        .next_frame(ARRIVAL_LIMIT)
        .expect("the frame arrives in time");

    assert_eq!(
        frame.payload,
        vec![1, 0x04, 0x03, 0x02, 0x01, 0, 0, 0, 2, 0x65, 0x88]
    );
}
