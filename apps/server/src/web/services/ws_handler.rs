use crate::hub::Hub;
use crate::web::{ViewerQuery, serve_viewer, viewer_request_from};
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Query, State};
use axum::response::Response;
use std::sync::Arc;

pub async fn ws_handler(
    upgrade: WebSocketUpgrade,
    State(hub): State<Arc<Hub>>,
    Query(query): Query<ViewerQuery>,
) -> Response {
    let request = viewer_request_from(&query);

    upgrade.on_upgrade(move |socket| serve_viewer(socket, hub, request))
}
