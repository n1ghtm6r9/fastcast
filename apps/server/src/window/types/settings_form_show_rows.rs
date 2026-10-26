use crate::settings::constants::{
    CAPTURE_FPS_SETTING, KEYFRAME_INTERVAL_SECONDS_SETTING, VIDEO_BITRATE_KBPS_SETTING,
};
use crate::window::{
    BACKEND_CHOICES, ENCODER_CHOICES, LEVEL_CHOICES, SettingsForm, choice_combo, monitor_combo,
    setting_row,
};
use eframe::egui::{DragValue, TextEdit, Ui};

impl SettingsForm {
    pub(crate) fn show_rows(&mut self, ui: &mut Ui) {
        let name_hint = format!(
            "Так ПК называется в списке на ТВ. Пусто — имя компьютера ({}).",
            self.computer_name
        );
        let edited = &mut self.edited;

        setting_row(ui, "Имя сервера", &name_hint, |ui| {
            ui.add(
                TextEdit::singleline(&mut edited.server_name)
                    .hint_text(&self.computer_name)
                    .desired_width(320.0),
            )
        });
        setting_row(
            ui,
            "Захват",
            "Авто: свой захват и NVENC, а если они не запустились — ffmpeg.",
            |ui| {
                choice_combo(
                    ui,
                    "capture_backend",
                    &mut edited.capture_backend,
                    &BACKEND_CHOICES,
                )
            },
        );
        setting_row(
            ui,
            "Кодировщик",
            "Для ffmpeg. Свой захват кодирует только NVENC, с AMF, QSV и x264 работает ffmpeg.",
            |ui| {
                choice_combo(
                    ui,
                    "video_encoder",
                    &mut edited.video_encoder,
                    &ENCODER_CHOICES,
                )
            },
        );
        setting_row(
            ui,
            "Виртуальный монитор",
            "На время трансляции единственным экраном становится монитор под разрешение ТВ.",
            |ui| ui.checkbox(&mut edited.virtual_display, ""),
        );
        setting_row(
            ui,
            "Монитор",
            "Какой монитор транслировать, если виртуального нет.",
            |ui| {
                monitor_combo(
                    ui,
                    &mut edited.capture_output,
                    &mut self.monitors,
                    self.list_monitors,
                )
            },
        );
        setting_row(
            ui,
            "Кадров в секунду",
            "Частота кадров, если виртуального монитора нет.",
            |ui| ui.add(DragValue::new(&mut edited.capture_fps).range(CAPTURE_FPS_SETTING.range())),
        );
        setting_row(
            ui,
            "Битрейт, кбит/с",
            "При 60 кадрах/с, выше растёт как √(fps/60). Дёргается по Wi-Fi — уменьши.",
            |ui| {
                ui.add(
                    DragValue::new(&mut edited.video_bitrate_kbps)
                        .speed(100)
                        .range(VIDEO_BITRATE_KBPS_SETTING.range()),
                )
            },
        );
        setting_row(
            ui,
            "10 бит",
            "HEVC в 10 битах, если ТВ умеет Main10. ffmpeg всегда кодирует 8 бит.",
            |ui| ui.checkbox(&mut edited.ten_bit_video, ""),
        );
        setting_row(
            ui,
            "Ключевой кадр, с",
            "Только для ffmpeg: новый ТВ ждёт ключевой кадр не дольше этого.",
            |ui| {
                ui.add(
                    DragValue::new(&mut edited.keyframe_interval_seconds)
                        .range(KEYFRAME_INTERVAL_SECONDS_SETTING.range()),
                )
            },
        );
        setting_row(
            ui,
            "Путь к ffmpeg",
            "Только для запасного пути. Пусто — ffmpeg.exe рядом с FastCast.exe или в PATH.",
            |ui| {
                ui.add(
                    TextEdit::singleline(&mut edited.ffmpeg_path)
                        .hint_text("ffmpeg")
                        .desired_width(320.0),
                )
            },
        );
        setting_row(
            ui,
            "Подробность журнала",
            "Подробная и полная пишут больше строк для поиска проблем.",
            |ui| choice_combo(ui, "log_level", &mut edited.log_level, &LEVEL_CHOICES),
        );
    }
}
