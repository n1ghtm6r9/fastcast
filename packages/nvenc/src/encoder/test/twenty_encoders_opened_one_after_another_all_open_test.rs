use super::{nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn twenty_encoders_opened_one_after_another_all_open() {
    let device = nvidia_device();
    let settings = settings(EncoderCodec::Hevc, 1920, 1080, 8);

    for attempt in 0..20 {
        let opened = Encoder::open(&device, &settings);

        assert!(opened.is_ok(), "attempt {}: {:?}", attempt, opened.err());
    }
}
