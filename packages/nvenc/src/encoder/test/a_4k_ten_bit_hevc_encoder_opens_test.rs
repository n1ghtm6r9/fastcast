use super::{nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_4k_ten_bit_hevc_encoder_opens() {
    let device = nvidia_device();

    let _encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 3840, 2160, 10)).unwrap();
}
