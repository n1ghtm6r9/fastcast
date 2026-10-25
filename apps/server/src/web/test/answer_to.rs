use crate::hub::Hub;
use crate::web::router;
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use std::sync::Arc;
use tower::ServiceExt;

pub async fn answer_to(request: Request<Body>) -> Response {
    router(Arc::new(Hub::new()))
        .oneshot(request)
        .await
        .expect("the router never fails")
}
