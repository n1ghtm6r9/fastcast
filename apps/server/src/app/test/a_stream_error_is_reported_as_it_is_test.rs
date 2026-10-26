use crate::app::AppError;
use crate::stream::StreamError;
use std::io;
use std::net::SocketAddr;

#[test]
fn a_stream_error_is_reported_as_it_is() {
    let failure = AppError::from(StreamError::Bind {
        address: SocketAddr::from(([0, 0, 0, 0], 3278)),
        source: io::Error::other("boom"),
    });

    assert_eq!(
        failure.to_string(),
        "could not listen on udp 0.0.0.0:3278: boom (is another fastcast already running?)"
    );
}
