use crate::hub::Hub;
use crate::web::{HEALTH_PATH, WS_PATH, health_handler, ws_handler};
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn router(hub: Arc<Hub>) -> Router {
    Router::new()
        .route(WS_PATH, get(ws_handler))
        .route(HEALTH_PATH, get(health_handler))
        .with_state(hub)
}
