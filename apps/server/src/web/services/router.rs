use crate::hub::Hub;
use crate::web::{ViewerTimings, router_with_timings};
use axum::Router;
use std::sync::Arc;

pub fn router(hub: Arc<Hub>) -> Router {
    router_with_timings(hub, ViewerTimings::default())
}
