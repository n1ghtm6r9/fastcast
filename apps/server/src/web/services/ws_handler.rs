use crate::hub::Hub;
use crate::web::{ViewerQuery, serve_viewer, viewer_request_from};
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Query, State};
use axum::http::header::ORIGIN;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use tracing::warn;

pub async fn ws_handler(
    upgrade: WebSocketUpgrade,
    headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    Query(query): Query<ViewerQuery>,
) -> Response {
    if let Some(origin) = headers.get(ORIGIN) {
        warn!("refused a websocket handshake from the page {origin:?}");

        return StatusCode::FORBIDDEN.into_response();
    }

    let request = viewer_request_from(&query);

    upgrade.on_upgrade(move |socket| serve_viewer(socket, hub, request))
}
