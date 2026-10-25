use crate::support::{
    FRAME_DATA, connect_viewer, hub_with_nvenc_config, next_config, next_frame, publish_frame,
    start_server,
};
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn a_viewer_that_sends_a_report_keeps_receiving_frames() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    next_config(&mut viewer).await;
    viewer
        .send(Message::text("{\"decoded\":300,\"dropped\":0}"))
        .await
        .expect("the report is sent");
    publish_frame(&hub, true, 0);

    let frame = next_frame(&mut viewer).await;

    assert!(frame.key);
    assert_eq!(frame.data, FRAME_DATA);
}
