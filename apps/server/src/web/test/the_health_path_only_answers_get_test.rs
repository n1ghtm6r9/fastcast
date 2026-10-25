use super::answer_to;
use axum::body::Body;
use axum::http::{Request, StatusCode};

#[tokio::test]
async fn the_health_path_only_answers_get() {
    let request = Request::builder()
        .method("POST")
        .uri("/api/health")
        .body(Body::empty())
        .expect("the request is valid");

    let response = answer_to(request).await;

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}
