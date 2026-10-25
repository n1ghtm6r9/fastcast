use axum::body::Body;
use axum::http::Request;

pub fn get_request(path: &str) -> Request<Body> {
    Request::builder()
        .uri(path)
        .body(Body::empty())
        .expect("the request is valid")
}
