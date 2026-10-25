use crate::hub::Hub;
use crate::web::{HEALTH_PATH, ViewerTimings, WS_PATH, health_handler, ws_handler};
use axum::routing::get;
use axum::{Extension, Router};
use std::sync::Arc;

pub fn router_with_timings(hub: Arc<Hub>, timings: ViewerTimings) -> Router {
    Router::new()
        .route(WS_PATH, get(ws_handler))
        .route(HEALTH_PATH, get(health_handler))
        .layer(Extension(timings))
        .with_state(hub)
}
