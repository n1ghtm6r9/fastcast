use crate::app::stop_server;
use crate::app::test::{ANSWER_LIMIT, TEST_COMPUTER_NAME};
use crate::server::ServerControl;
use crate::settings::{SavedSettings, settings_of};
use crate::stream::StreamError;
use std::future;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

#[test]
fn a_server_that_does_not_stop_does_not_keep_fastcast_open() {
    let runtime = Runtime::new().expect("the runtime starts");
    let settings = settings_of(&SavedSettings::default(), TEST_COMPUTER_NAME);
    let control = ServerControl::start_with(runtime.handle(), settings, |_, _, _| {
        future::pending::<Result<(), StreamError>>()
    });
    let (closed, done) = mpsc::channel();

    thread::spawn(move || {
        stop_server(runtime, control, Duration::from_millis(100));
        let _ = closed.send(());
    });

    assert_eq!(done.recv_timeout(ANSWER_LIMIT), Ok(()));
}
