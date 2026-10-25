use crate::support::{
    FRAME_DATA, connect_viewer, hub_with_nvenc_config, next_config, next_message, start_server,
};
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn a_binary_message_is_a_flag_byte_a_timestamp_and_the_access_unit() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    next_config(&mut viewer).await;
    hub.publish(true, 0x0102_0304, &FRAME_DATA);

    let Message::Binary(wire) = next_message(&mut viewer).await else {
        panic!("a frame is a binary message");
    };

    assert_eq!(
        wire.to_vec(),
        vec![1, 0x04, 0x03, 0x02, 0x01, 0, 0, 0, 2, 0x65, 0x88]
    );
}
