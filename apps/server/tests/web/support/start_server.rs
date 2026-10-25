use crate::support::{RunningServer, start_server_with_timings};
use app::hub::Hub;
use app::web::ViewerTimings;
use std::sync::Arc;

pub async fn start_server(hub: Arc<Hub>) -> RunningServer {
    start_server_with_timings(hub, ViewerTimings::default()).await
}
