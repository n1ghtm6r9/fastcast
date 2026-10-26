use crate::display::DisplayMode;
use crate::hub::ViewerRequest;
use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;
use media::VideoCodec;

#[test]
fn the_status_shows_what_the_connected_tv_asked_for() {
    let watched = ServerStatus {
        viewers: 1,
        request: ViewerRequest {
            mode: Some(DisplayMode {
                width: 3840,
                height: 2160,
                refresh_hz: 120,
            }),
            codecs: vec![VideoCodec::Av1, VideoCodec::Hevc, VideoCodec::Avc],
            ten_bit_codecs: vec![VideoCodec::Av1, VideoCodec::Hevc],
        },
        ..running_status()
    };
    let (window, _probe) = test_window(SavedSettings::default(), watched, saving_works);

    let harness = window_harness(window);

    harness
        .get_by_label("ТВ просит 3840×2160 при 120 Гц, декодирует AV1 10 бит, HEVC 10 бит, H.264");
}
