use super::{flv_stream, sequence_header_tag};
use crate::capture::pump_frames;
use crate::fixtures::nvenc_2560x1440_avcc;
use crate::hub::Hub;
use std::io::Cursor;

#[tokio::test]
async fn the_configured_rate_is_what_the_config_announces() {
    let hub = Hub::new();
    let stream = flv_stream(&[sequence_header_tag(&nvenc_2560x1440_avcc())]);

    pump_frames(Cursor::new(stream), &hub, 144)
        .await
        .expect("the stream is valid");

    assert_eq!(hub.join().config.expect("a config").fps, 144);
}
