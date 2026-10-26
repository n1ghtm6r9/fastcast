use crate::hub::Hub;
use crate::server::ServerStatus;
use std::sync::Arc;
use tokio::select;
use tokio::sync::watch;

pub async fn forward_hub_status(hub: Arc<Hub>, status: watch::Sender<ServerStatus>) {
    let mut viewers = hub.watch_viewers();
    let mut requests = hub.watch_requests();
    let mut reports = hub.watch_reports();

    loop {
        status.send_modify(|status| {
            status.viewers = *viewers.borrow_and_update();
            status.request = requests.borrow_and_update().clone();
            status.report = reports.borrow_and_update().clone();
        });

        let open = select! {
            changed = viewers.changed() => changed.is_ok(),
            changed = requests.changed() => changed.is_ok(),
            changed = reports.changed() => changed.is_ok(),
        };

        if !open {
            return;
        }
    }
}
