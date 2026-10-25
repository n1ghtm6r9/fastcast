use super::{answer_to, get_request};
use axum::http::StatusCode;

#[tokio::test]
async fn unknown_paths_are_not_found() {
    for path in [
        "/",
        "/index.html",
        "/assets/app.js",
        "/api",
        "/api/health/extra",
        "/ws/extra",
    ] {
        let response = answer_to(get_request(path)).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
    }
}
