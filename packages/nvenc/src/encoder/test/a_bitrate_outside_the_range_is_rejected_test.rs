use super::{gradient_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec, NvencError};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_bitrate_outside_the_range_is_rejected() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();

    let results = [encoder.set_bitrate(0), encoder.set_bitrate(800_001)];

    assert_eq!(
        results,
        [
            Err(NvencError::InvalidSettings),
            Err(NvencError::InvalidSettings)
        ]
    );
    assert!(
        encoder
            .encode(&gradient_texture(&device, 1920, 1080, 8, 0), false)
            .is_ok()
    );
}
