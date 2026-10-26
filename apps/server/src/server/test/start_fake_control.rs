use crate::server::ServerControl;
use crate::server::test::{FakeEvent, fake_server, settings_named};
use tokio::runtime::Handle;
use tokio::sync::mpsc;

pub fn start_fake_control(name: &str) -> (ServerControl, mpsc::UnboundedReceiver<FakeEvent>) {
    let (events, seen) = mpsc::unbounded_channel();
    let control = ServerControl::start_with(
        &Handle::current(),
        settings_named(name),
        move |settings, stop, status| fake_server(settings, stop, status, events.clone()),
    );

    (control, seen)
}
