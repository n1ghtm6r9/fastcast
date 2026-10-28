use crate::hub::ViewerRequest;
use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;
use media::VideoCodec;

#[test]
fn a_tv_without_a_screen_mode_asks_for_the_set_picture() {
    let watched = ServerStatus {
        viewers: 1,
        request: ViewerRequest {
            mode: None,
            codecs: vec![VideoCodec::Avc],
            ten_bit_codecs: Vec::new(),
            noise: false,
        },
        ..running_status()
    };
    let (window, _probe) = test_window(SavedSettings::default(), watched, saving_works);

    let harness = window_harness(window);

    harness.get_by_label("ТВ просит картинку по настройкам, декодирует H.264");
}
