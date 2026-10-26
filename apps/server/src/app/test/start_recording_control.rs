use crate::server::ServerControl;
use crate::settings::{SavedSettings, Settings, settings_of};
use tokio::runtime::Handle;
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub fn start_recording_control() -> (ServerControl, UnboundedReceiver<Settings>) {
    let (started, receiver) = mpsc::unbounded_channel();
    let first = settings_of(&SavedSettings::default());
    let control = ServerControl::start_with(&Handle::current(), first, move |settings, stop, _| {
        let started = started.clone();

        async move {
            let _ = started.send(settings);
            let _ = stop.await;
            Ok(())
        }
    });

    (control, receiver)
}
