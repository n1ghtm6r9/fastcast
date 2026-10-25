use nvenc::{EncodedFrame, Encoder, EncoderCodec, EncoderSettings, NvencError};
use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D};

#[test]
fn the_crate_root_exports_the_encoder_api() {
    let _open: fn(&ID3D11Device, &EncoderSettings) -> Result<Encoder, NvencError> = Encoder::open;
    let _encode: fn(&mut Encoder, &ID3D11Texture2D, bool) -> Result<EncodedFrame, NvencError> =
        Encoder::encode;
    let _set_bitrate: fn(&mut Encoder, u32) -> Result<(), NvencError> = Encoder::set_bitrate;
    let settings = EncoderSettings {
        codec: EncoderCodec::Hevc,
        width: 1920,
        height: 1080,
        fps: 60,
        bitrate_kbps: 50_000,
        bit_depth: 8,
        preset: 4,
    };
    let frame = EncodedFrame {
        data: vec![0, 0, 1],
        keyframe: true,
    };

    assert_eq!(settings.codec, EncoderCodec::Hevc);
    assert!(frame.keyframe);
    assert_ne!(NvencError::TextureMismatch, NvencError::TextureLimit);
}
