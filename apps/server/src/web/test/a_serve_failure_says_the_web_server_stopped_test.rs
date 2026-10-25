use crate::web::WebError;
use std::io;

#[test]
fn a_serve_failure_says_the_web_server_stopped() {
    let failure = WebError::Serve(io::Error::other("boom"));

    assert_eq!(failure.to_string(), "the web server stopped: boom");
}
