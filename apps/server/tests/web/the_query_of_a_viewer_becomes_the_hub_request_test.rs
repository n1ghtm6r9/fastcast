use crate::support::{
    connect_viewer_asking, hub_with_nvenc_config, start_server, wait_for_request,
};
use app::display::DisplayMode;
use app::hub::ViewerRequest;
use media::VideoCodec;

#[tokio::test]
async fn the_query_of_a_viewer_becomes_the_hub_request() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let _viewer = connect_viewer_asking(
        server.address,
        "width=3840&height=2160&fps=60&codecs=hevc,avc",
    )
    .await;

    wait_for_request(
        &hub,
        &ViewerRequest {
            mode: Some(DisplayMode {
                width: 3840,
                height: 2160,
                refresh_hz: 60,
            }),
            codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        },
    )
    .await;
}
