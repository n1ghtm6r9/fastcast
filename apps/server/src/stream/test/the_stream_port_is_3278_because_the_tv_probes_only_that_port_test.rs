use crate::stream::STREAM_PORT;

#[test]
fn the_stream_port_is_3278_because_the_tv_probes_only_that_port() {
    assert_eq!(STREAM_PORT, 3278);
}
