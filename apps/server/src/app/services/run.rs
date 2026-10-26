use crate::app::{
    AppError, init_dotenv, init_logger, log_ffmpeg, log_stream_address, shutdown_signal,
};
use crate::capture::{FfmpegBackend, supervise_capture};
use crate::hub::Hub;
use crate::settings::load_settings;
use crate::stream::{
    EndpointOptions, STREAM_PORT, StreamEndpoint, bind_stream_socket, stream_address,
};
use std::sync::Arc;
use tracing::warn;

pub async fn run() -> Result<(), AppError> {
    init_dotenv()?;
    init_logger();

    let settings = load_settings()?;
    let hub = Arc::new(Hub::new());
    let socket = bind_stream_socket(stream_address(STREAM_PORT))?;

    log_stream_address(STREAM_PORT);
    log_ffmpeg(&settings.ffmpeg_path);

    let endpoint = StreamEndpoint::start(
        socket,
        hub.clone(),
        EndpointOptions::new(settings.server_name.clone(), settings.capture_fps),
    )?;
    let backend = Arc::new(FfmpegBackend::new(settings));
    let supervisor = tokio::spawn(supervise_capture(hub, backend));

    shutdown_signal().await;

    supervisor.abort();
    let _ = supervisor.await;

    if let Err(error) = tokio::task::spawn_blocking(move || endpoint.stop()).await {
        warn!("the stream endpoint did not stop cleanly: {error}");
    }

    Ok(())
}
