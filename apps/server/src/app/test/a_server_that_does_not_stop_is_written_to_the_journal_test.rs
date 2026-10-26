use crate::app::stop_server;
use crate::app::test::{ANSWER_LIMIT, TEST_COMPUTER_NAME};
use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::server::ServerControl;
use crate::settings::{LogLevel, SavedSettings, settings_of};
use crate::stream::StreamError;
use std::future;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

#[test]
fn a_server_that_does_not_stop_is_written_to_the_journal() {
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, _level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);
    let runtime = Runtime::new().expect("the runtime starts");
    let settings = settings_of(&SavedSettings::default(), TEST_COMPUTER_NAME);
    let control = ServerControl::start_with(runtime.handle(), settings, |_, _, _| {
        future::pending::<Result<(), StreamError>>()
    });
    let (closed, done) = mpsc::channel();

    thread::spawn(move || {
        tracing::subscriber::with_default(subscriber, || {
            stop_server(runtime, control, Duration::from_millis(100));
        });
        let _ = closed.send(());
    });

    assert_eq!(done.recv_timeout(ANSWER_LIMIT), Ok(()));
    let shown = lines.lines();
    assert!(
        shown.iter().any(|line| line
            .ends_with(" WARN the server did not stop within 100 ms, FastCast closes anyway")),
        "{shown:?}"
    );
}
