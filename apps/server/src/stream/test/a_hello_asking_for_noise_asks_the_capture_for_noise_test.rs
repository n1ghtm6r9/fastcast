use crate::stream::viewer_request_of;
use wire::Hello;

#[test]
fn a_hello_asking_for_noise_asks_the_capture_for_noise() {
    let hello = Hello {
        nonce: 1,
        width: 2560,
        height: 1440,
        fps: 120,
        codecs: Vec::new(),
        noise: true,
    };

    assert!(viewer_request_of(&hello, 60).noise);
}
