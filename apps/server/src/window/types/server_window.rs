use crate::window::{JournalView, REPAINT_INTERVAL, ServerLink, SettingsForm};
use eframe::egui::Ui;

pub struct ServerWindow<'a> {
    pub(crate) link: ServerLink<'a>,
    pub(crate) journal: JournalView,
    pub(crate) form: SettingsForm,
}

impl<'a> ServerWindow<'a> {
    pub fn new(link: ServerLink<'a>, journal: JournalView, form: SettingsForm) -> Self {
        Self {
            link,
            journal,
            form,
        }
    }
}

impl eframe::App for ServerWindow<'_> {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
        ui.ctx().request_repaint_after(REPAINT_INTERVAL);
    }
}
