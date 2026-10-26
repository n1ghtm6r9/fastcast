use crate::app::{
    STOP_LIMIT, WINDOW_TITLE, apply_saved_settings, show_problem, stop_server, window_options,
    window_problem_text,
};
use crate::journal::Journal;
use crate::server::ServerControl;
use crate::settings::{SavedSettings, settings_of};
use crate::window::{JournalView, ServerLink, ServerWindow, SettingsForm, list_monitors};
use std::path::Path;
use tokio::runtime::Runtime;
use tracing::error;

pub fn serve_in_window(
    runtime: Runtime,
    journal: &Journal,
    saved: SavedSettings,
    settings_file: &Path,
    journal_file: &Path,
) {
    let control = ServerControl::start(runtime.handle(), settings_of(&saved));
    let link = ServerLink::new(control.watch_status(), |edited| {
        apply_saved_settings(&control, &journal.level, settings_file, edited)
    });
    let window = ServerWindow::new(
        link,
        JournalView::new(journal.lines.clone(), journal_file.to_owned()),
        SettingsForm::new(saved, list_monitors),
    );

    if let Err(problem) = eframe::run_native(
        WINDOW_TITLE,
        window_options(),
        Box::new(|_| Ok(Box::new(window))),
    ) {
        error!("the window cannot open: {problem}");
        show_problem(&window_problem_text(problem, journal_file));
    }

    stop_server(runtime, control, STOP_LIMIT);
}
