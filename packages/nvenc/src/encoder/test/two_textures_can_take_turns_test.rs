use super::{gradient_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn two_textures_can_take_turns() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let textures = [
        gradient_texture(&device, 1920, 1080, 8, 0),
        gradient_texture(&device, 1920, 1080, 8, 64),
    ];

    for turn in 0..10 {
        let frame = encoder.encode(&textures[turn % 2], false);

        assert!(frame.is_ok(), "frame {}: {:?}", turn, frame.err());
    }
}
