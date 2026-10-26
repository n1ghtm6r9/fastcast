use crate::settings::{BackendChoice, EncoderChoice, LogLevel, SavedSettings, VideoEncoder};
use crate::window::test::{
    applied_settings, choose_option, press_button, running_status, saving_works, set_number,
    test_window, toggle_box, type_into, window_harness,
};

#[test]
fn every_setting_in_the_form_reaches_the_server() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя сервера", "Gaming PC");
    choose_option(&mut harness, "Захват", "Только ffmpeg");
    choose_option(&mut harness, "Кодировщик", "x264");
    toggle_box(&mut harness, "Виртуальный монитор");
    choose_option(&mut harness, "Монитор", "1: DISPLAY2, 1920×1080");
    set_number(&mut harness, "Кадров в секунду", 120.0);
    set_number(&mut harness, "Битрейт, кбит/с", 56000.0);
    toggle_box(&mut harness, "10 бит");
    set_number(&mut harness, "Ключевой кадр, с", 2.0);
    type_into(&mut harness, "Путь к ffmpeg", "D:\\tools\\ffmpeg.exe");
    choose_option(&mut harness, "Подробность журнала", "Подробная");
    press_button(&mut harness, "Применить");

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            server_name: "Gaming PC".to_owned(),
            capture_backend: BackendChoice::Ffmpeg,
            video_encoder: EncoderChoice::Fixed(VideoEncoder::X264),
            virtual_display: false,
            capture_output: 1,
            capture_fps: 120,
            video_bitrate_kbps: 56000,
            ten_bit_video: false,
            keyframe_interval_seconds: 2,
            ffmpeg_path: "D:\\tools\\ffmpeg.exe".to_owned(),
            log_level: LogLevel::Debug,
        }]
    );
}
