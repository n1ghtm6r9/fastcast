use crate::capture::rescaled;
use crate::display::ScreenScaler;
use crate::hub::Hub;
use std::convert::Infallible;
use std::future::pending;
use tokio::sync::broadcast::error::RecvError;
use tracing::info;

pub async fn serve_screen_scale<S: ScreenScaler>(hub: &Hub, output: u32, scaler: &S) -> Infallible {
    let mut asks = hub.watch_scale_asks();
    let show = |step: i32| {
        let scale = rescaled(scaler, output, step);

        if let Some(scale) = scale {
            info!("output {output} is scaled to {} %", scale.percent);
        }

        hub.show_scale(scale);
    };

    show(0);

    loop {
        match asks.recv().await {
            Ok(step) => show(step),
            Err(RecvError::Lagged(_)) => {}
            Err(RecvError::Closed) => pending::<()>().await,
        }
    }
}
