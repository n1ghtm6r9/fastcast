use crate::app::AppError;
use crate::web::WebError;
use std::io;

#[test]
fn a_web_error_is_reported_as_it_is() {
    let failure = AppError::from(WebError::Serve(io::Error::other("boom")));

    assert_eq!(failure.to_string(), "the web server stopped: boom");
}
