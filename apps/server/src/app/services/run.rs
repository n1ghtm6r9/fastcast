use crate::app::{AppError, init_dotenv, init_logger, log_urls, shutdown_signal};
use crate::capture::{FfmpegBackend, supervise_capture, with_located_ffmpeg};
use crate::discovery::{discovery_address, run_discovery};
use crate::hub::Hub;
use crate::settings::load_settings;
use crate::web::{bind_listener, serve_web};
use std::sync::Arc;

pub async fn run() -> Result<(), AppError> {
    init_dotenv();
    init_logger();

    let settings = with_located_ffmpeg(load_settings()?);
    let hub = Arc::new(Hub::new());
    let listener = bind_listener(&settings.web_host, settings.web_port).await?;

    log_urls(&settings.web_host, settings.web_port);

    let responder = tokio::spawn(run_discovery(
        discovery_address(settings.discovery_port),
        settings.server_name.clone(),
        settings.web_port,
    ));
    let backend = Arc::new(FfmpegBackend::new(settings));
    let supervisor = tokio::spawn(supervise_capture(hub.clone(), backend));
    let served = serve_web(listener, hub, shutdown_signal()).await;

    supervisor.abort();
    let _ = supervisor.await;

    responder.abort();
    let _ = responder.await;

    served?;

    Ok(())
}
