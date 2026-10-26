use crate::server::test::{BUSY_NAME, FAKE_ADDRESS, FakeEvent, busy_port_error};
use crate::server::{ServerState, ServerStatus};
use crate::settings::Settings;
use crate::stream::StreamError;
use tokio::sync::{mpsc, oneshot, watch};

pub async fn fake_server(
    settings: Settings,
    stop: oneshot::Receiver<()>,
    status: watch::Sender<ServerStatus>,
    events: mpsc::UnboundedSender<FakeEvent>,
) -> Result<(), StreamError> {
    let name = settings.server_name;
    let _ = events.send(FakeEvent::Started {
        name: name.clone(),
        status: status.borrow().clone(),
    });

    if name == BUSY_NAME {
        return Err(busy_port_error());
    }

    status.send_modify(|status| {
        status.state = ServerState::Running {
            address: FAKE_ADDRESS,
        };
        status.viewers = 1;
    });

    let _ = stop.await;
    let _ = events.send(FakeEvent::Stopped { name });

    Ok(())
}
