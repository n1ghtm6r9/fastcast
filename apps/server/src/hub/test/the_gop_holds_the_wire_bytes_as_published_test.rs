use crate::fixtures::hub_with_config;
use crate::hub::encode_wire_frame;

#[test]
fn the_gop_holds_the_wire_bytes_as_published() {
    let hub = hub_with_config();

    hub.publish(true, 3, &[4]);
    hub.publish(false, 4, &[5, 6]);

    let gop = hub.join().gop;

    assert_eq!(gop[0].wire, encode_wire_frame(true, 3, &[4]));
    assert_eq!(gop[1].wire, encode_wire_frame(false, 4, &[5, 6]));
}
