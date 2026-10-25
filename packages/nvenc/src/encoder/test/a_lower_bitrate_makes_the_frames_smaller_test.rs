use super::{noise_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_lower_bitrate_makes_the_frames_smaller() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let textures: Vec<_> = (0..4)
        .map(|seed| noise_texture(&device, 1920, 1080, seed))
        .collect();
    let mean_size_after_settling = |encoder: &mut Encoder| {
        let sizes: Vec<usize> = (0..12)
            .map(|turn| {
                encoder
                    .encode(&textures[turn % 4], false)
                    .unwrap()
                    .data
                    .len()
            })
            .collect();
        sizes[6..].iter().sum::<usize>() / 6
    };

    let before = mean_size_after_settling(&mut encoder);
    encoder.set_bitrate(5_000).unwrap();
    let after = mean_size_after_settling(&mut encoder);

    assert!(
        after * 3 < before,
        "{} bytes per frame before, {} after",
        before,
        after
    );
}
