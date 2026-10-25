use crate::encoder::{EncoderCodec, EncoderSettings};
use crate::ffi::{FC_CODEC_H264, FC_CODEC_HEVC, FcSettings};

pub fn fc_settings_of(settings: &EncoderSettings) -> FcSettings {
    FcSettings {
        codec: match settings.codec {
            EncoderCodec::Avc => FC_CODEC_H264,
            EncoderCodec::Hevc => FC_CODEC_HEVC,
        },
        width: settings.width,
        height: settings.height,
        fps: settings.fps,
        bitrate_kbps: settings.bitrate_kbps,
        bit_depth: u32::from(settings.bit_depth),
        preset: u32::from(settings.preset),
    }
}
