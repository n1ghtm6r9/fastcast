use crate::hub::Hub;
use crate::web::serve_viewer;
use axum::extract::State;
use axum::extract::ws::WebSocketUpgrade;
use axum::response::Response;
use std::sync::Arc;

pub async fn ws_handler(upgrade: WebSocketUpgrade, State(hub): State<Arc<Hub>>) -> Response {
    upgrade.on_upgrade(move |socket| serve_viewer(socket, hub))
}
