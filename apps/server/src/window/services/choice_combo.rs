use eframe::egui::{ComboBox, Response, Ui};

pub fn choice_combo<T: PartialEq + Copy>(
    ui: &mut Ui,
    id_salt: &str,
    value: &mut T,
    choices: &[(T, &str)],
) -> Response {
    let selected = choices
        .iter()
        .find(|(choice, _)| choice == value)
        .map_or("", |(_, name)| *name);

    ComboBox::from_id_salt(id_salt)
        .selected_text(selected)
        .show_ui(ui, |ui| {
            for (choice, name) in choices {
                ui.selectable_value(value, *choice, *name);
            }
        })
        .response
}
