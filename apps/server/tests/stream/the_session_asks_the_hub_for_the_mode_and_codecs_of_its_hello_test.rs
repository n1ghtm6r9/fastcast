use crate::support::{TestViewer, a_hello, hub_with_config, start_endpoint, wait_for_request};
use app::display::DisplayMode;
use app::hub::ViewerRequest;
use media::VideoCodec;
use wire::{Codec, SESSION_TIMEOUT};

#[test]
fn the_session_asks_the_hub_for_the_mode_and_codecs_of_its_hello() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let _viewer = TestViewer::connect(
        endpoint.local_addr(),
        a_hello(3840, 2160, 120, &[Codec::Hevc, Codec::Avc]),
    );

    wait_for_request(
        &hub,
        &ViewerRequest {
            mode: Some(DisplayMode {
                width: 3840,
                height: 2160,
                refresh_hz: 120,
            }),
            codecs: vec![VideoCodec::Hevc, VideoCodec::Avc],
            ten_bit_codecs: Vec::new(),
        },
    );
}
