use crate::app::stop_server;
use crate::app::test::{ANSWER_LIMIT, TEST_COMPUTER_NAME};
use crate::server::ServerControl;
use crate::settings::{SavedSettings, settings_of};
use std::sync::mpsc;
use tokio::runtime::Runtime;

#[test]
fn closing_the_window_stops_the_server() {
    let runtime = Runtime::new().expect("the runtime starts");
    let (stopped, told) = mpsc::channel();
    let settings = settings_of(&SavedSettings::default(), TEST_COMPUTER_NAME);
    let control = ServerControl::start_with(runtime.handle(), settings, move |_, stop, _| {
        let stopped = stopped.clone();

        async move {
            let _ = stop.await;
            let _ = stopped.send(());
            Ok(())
        }
    });

    stop_server(runtime, control, ANSWER_LIMIT);

    assert_eq!(told.try_recv(), Ok(()));
}
