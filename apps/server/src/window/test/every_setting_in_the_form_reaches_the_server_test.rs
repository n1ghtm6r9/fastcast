use crate::settings::{CodecChoice, FrameRate, LogLevel, SavedSettings};
use crate::window::test::{
    applied_settings, choose_option, finish_typing, open_journal, running_status, saving_works,
    slide_to, test_window, tick, type_into, window_harness,
};

#[test]
fn every_setting_in_the_form_reaches_the_server() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя на ТВ", "Gaming PC");
    finish_typing(&mut harness);
    choose_option(&mut harness, "Экран", "1: DISPLAY2, 1920×1080");
    choose_option(&mut harness, "Кадров/с", "120");
    choose_option(&mut harness, "Кодек", "HEVC");
    slide_to(&mut harness, "Битрейт", 80.0);
    tick(&mut harness, "Отладка");
    open_journal(&mut harness);
    choose_option(&mut harness, "Подробность", "Подробная");

    assert_eq!(
        applied_settings(&probe).last(),
        Some(&SavedSettings {
            server_name: "Gaming PC".to_owned(),
            virtual_display: false,
            capture_output: 1,
            frame_rate: FrameRate::Fps120,
            video_codec: CodecChoice::Hevc,
            video_bitrate_mbps: 80,
            log_level: LogLevel::Debug,
            debug: true,
        })
    );
}
