use crate::journal::{JOURNAL_LINE_LIMIT, JournalLines};
use crate::server::ServerStatus;
use crate::settings::{SavedSettings, SettingsFileError};
use crate::window::test::{TEST_COMPUTER_NAME, TEST_JOURNAL_FILE, WindowProbe};
use crate::window::{JournalView, Monitor, ServerLink, ServerWindow, SettingsForm};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::watch;

pub fn test_window_listing(
    saved: SavedSettings,
    status: ServerStatus,
    saving: fn() -> Result<(), SettingsFileError>,
    list_monitors: fn() -> Vec<Monitor>,
) -> (ServerWindow<'static>, WindowProbe) {
    let (status, watched) = watch::channel(status);
    let applied = Rc::new(RefCell::new(Vec::new()));
    let lines = Arc::new(JournalLines::new(JOURNAL_LINE_LIMIT));
    let link = ServerLink::new(watched, {
        let applied = applied.clone();

        move |settings: &SavedSettings| {
            applied.borrow_mut().push(settings.clone());
            saving()
        }
    });
    let window = ServerWindow::new(
        link,
        JournalView::new(lines.clone(), PathBuf::from(TEST_JOURNAL_FILE)),
        SettingsForm::new(saved, TEST_COMPUTER_NAME.to_owned(), list_monitors),
    );

    (
        window,
        WindowProbe {
            applied,
            status,
            lines,
        },
    )
}
