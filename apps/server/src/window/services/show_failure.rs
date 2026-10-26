use eframe::egui::Ui;

pub fn show_failure(ui: &mut Ui, reason: &str) -> bool {
    ui.horizontal_wrapped(|ui| {
        ui.colored_label(ui.visuals().error_fg_color, reason);
        ui.button("Запустить снова").clicked()
    })
    .inner
}
