use crate::display::DisplayMode;
use crate::hub::{Hub, ViewerRequest};
use crate::server::test::status_where;
use crate::server::{ServerStatus, forward_hub_status};
use media::VideoCodec;
use std::sync::Arc;
use tokio::sync::watch;

#[tokio::test]
async fn the_status_shows_what_the_tv_asked_for() {
    let hub = Arc::new(Hub::new());
    let (status, mut watched) = watch::channel(ServerStatus::default());
    let forwarding = tokio::spawn(forward_hub_status(hub.clone(), status));
    let request = ViewerRequest {
        mode: Some(DisplayMode {
            width: 3840,
            height: 2160,
            refresh_hz: 120,
        }),
        codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
        ten_bit_codecs: vec![VideoCodec::Hevc],
    };

    hub.request(request.clone());

    assert_eq!(
        status_where(&mut watched, |status| status.request
            != ViewerRequest::default())
        .await
        .request,
        request
    );

    forwarding.abort();
}
