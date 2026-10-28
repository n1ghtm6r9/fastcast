use super::default_settings;
use crate::capture::{CaptureBackend, NativeBackend};
use crate::hub::ViewerRequest;

#[tokio::test]
async fn the_native_backend_prepares_noise_when_the_tv_asks_for_it() {
    let request = ViewerRequest {
        noise: true,
        ..ViewerRequest::default()
    };

    let pipeline = NativeBackend::new(default_settings())
        .prepare(&request)
        .await
        .expect("the native backend prepares without any work");

    assert!(pipeline.noise);
}
