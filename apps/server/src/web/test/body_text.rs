use axum::body::to_bytes;
use axum::response::Response;

pub async fn body_text(response: Response) -> String {
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("the body can be read");

    String::from_utf8(bytes.to_vec()).expect("the body is text")
}
