use super::{answer_to, body_text, get_request};
use axum::http::StatusCode;

#[tokio::test]
async fn the_health_check_answers_ok() {
    let response = answer_to(get_request("/api/health")).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(body_text(response).await, "ok");
}
