use super::{nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn an_av1_encoder_opens_on_the_nvidia_adapter() {
    let device = nvidia_device();

    let _encoder = Encoder::open(&device, &settings(EncoderCodec::Av1, 1920, 1080, 8)).unwrap();
}
