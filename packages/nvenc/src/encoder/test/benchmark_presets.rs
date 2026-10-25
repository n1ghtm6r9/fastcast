use super::{gradient_texture, nvidia_device};
use crate::encoder::{Encoder, EncoderCodec, EncoderSettings};
use std::time::Instant;

pub fn benchmark_presets(bit_depth: u8) {
    let device = nvidia_device();
    let textures = [
        gradient_texture(&device, 3840, 2160, bit_depth, 0),
        gradient_texture(&device, 3840, 2160, bit_depth, 64),
    ];

    for preset in 1..=7 {
        let settings = EncoderSettings {
            codec: EncoderCodec::Hevc,
            width: 3840,
            height: 2160,
            fps: 120,
            bitrate_kbps: 150_000,
            bit_depth,
            preset,
        };
        let mut encoder = Encoder::open(&device, &settings).unwrap();
        for turn in 0..10 {
            encoder.encode(&textures[turn % 2], false).unwrap();
        }

        let milliseconds: Vec<f64> = (0..60)
            .map(|turn| {
                let started = Instant::now();
                encoder.encode(&textures[turn % 2], false).unwrap();
                started.elapsed().as_secs_f64() * 1000.0
            })
            .collect();
        let mean = milliseconds.iter().sum::<f64>() / milliseconds.len() as f64;
        let worst = milliseconds.iter().copied().fold(0.0, f64::max);

        println!(
            "4K {bit_depth}-bit HEVC preset P{preset}: mean {mean:.2} ms, worst {worst:.2} ms"
        );
    }
}
