use crate::app::{data_directory, log_panics, serve_in_window, show_problem, window_problem_text};
use crate::journal::{JOURNAL_FILE_NAME, start_journal};
use crate::settings::constants::SETTINGS_FILE_NAME;
use crate::settings::{SavedSettings, read_saved_settings};
use tokio::runtime::Runtime;
use tracing::{error, warn};

pub fn run() {
    let directory = data_directory();
    let journal_file = directory.join(JOURNAL_FILE_NAME);
    let settings_file = directory.join(SETTINGS_FILE_NAME);
    let (saved, unread) = match read_saved_settings(&settings_file) {
        Ok(saved) => (saved, None),
        Err(problem) => (SavedSettings::default(), Some(problem)),
    };
    let journal = start_journal(saved.log_level, &journal_file);

    log_panics();
    if let Some(problem) = unread {
        warn!("{problem}, the defaults are used");
    }

    match Runtime::new() {
        Ok(runtime) => serve_in_window(runtime, &journal, saved, &settings_file, &journal_file),
        Err(problem) => {
            error!("the server runtime cannot start: {problem}");
            show_problem(&window_problem_text(problem, &journal_file));
        }
    }
}
