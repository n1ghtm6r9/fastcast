use crate::app::{lan_ip, log_ffmpeg, log_stream_address};
use crate::capture::{FfmpegBackend, NativeBackend, SwitchingBackend, supervise_capture};
use crate::display::SystemScaler;
use crate::hub::Hub;
use crate::server::{ServerState, ServerStatus, diagnose_run_of, forward_hub_status};
use crate::settings::Settings;
use crate::stream::{
    EndpointOptions, FALLBACK_STREAM_FPS, StreamEndpoint, StreamError, bind_stream_socket,
    stream_address,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{oneshot, watch};
use tokio::task::{JoinSet, spawn_blocking};
use tracing::{info, warn};

pub async fn run_server(
    settings: Settings,
    port: u16,
    stop: oneshot::Receiver<()>,
    status: watch::Sender<ServerStatus>,
) -> Result<(), StreamError> {
    let hub = Arc::new(Hub::new());

    if let Some(run) = diagnose_run_of(&settings, SystemTime::now()) {
        info!("debug: the TV will be asked to diagnose itself (run {run})");
        hub.diagnose(run);
    }

    let socket = bind_stream_socket(stream_address(port))?;
    let endpoint = StreamEndpoint::start(
        socket,
        hub.clone(),
        EndpointOptions::new(settings.server_name.clone(), FALLBACK_STREAM_FPS),
    )?;
    let address = SocketAddr::new(
        lan_ip().unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        endpoint.local_addr().port(),
    );
    let backend = Arc::new(SwitchingBackend::new(
        Arc::new(NativeBackend::new(settings.clone())),
        Arc::new(FfmpegBackend::new(settings.clone())),
    ));
    let mut tasks = JoinSet::new();

    log_stream_address(address);
    log_ffmpeg();
    tasks.spawn(supervise_capture(
        hub.clone(),
        backend,
        Arc::new(SystemScaler),
    ));
    tasks.spawn(forward_hub_status(hub, status.clone()));
    status.send_modify(|status| status.state = ServerState::Running { address });

    let _ = stop.await;

    tasks.shutdown().await;
    if let Err(error) = spawn_blocking(move || endpoint.stop()).await {
        warn!("the stream endpoint did not stop cleanly: {error}");
    }
    info!("the server is stopped");

    Ok(())
}
