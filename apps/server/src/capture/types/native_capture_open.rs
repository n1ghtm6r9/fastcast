use crate::capture::{
    CaptureTarget, FrameClock, NATIVE_ENCODER_PRESET, NativeCapture, NativeError, encoder_codec_of,
    nvidia_output_for,
};
use crate::hub::Hub;
use desktop::{BitDepth, Converter, Duplication, GpuDevice, list_outputs};
use media::VideoCodec;
use nvenc::{Encoder, EncoderSettings};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

impl<'a> NativeCapture<'a> {
    pub fn open(
        hub: &'a Hub,
        codec: VideoCodec,
        target: &CaptureTarget,
        bitrate_kbps: u32,
        stop: &'a AtomicBool,
    ) -> Result<Self, NativeError> {
        let outputs = list_outputs().map_err(NativeError::Desktop)?;
        let output = nvidia_output_for(target, &outputs)?.clone();
        let device = GpuDevice::new(output.adapter_luid).map_err(NativeError::Desktop)?;
        let duplication = Duplication::new(&device, &output).map_err(NativeError::Desktop)?;
        let size = (duplication.width(), duplication.height());
        let converter = Converter::new(&device, size.0, size.1, BitDepth::Eight)
            .map_err(NativeError::Desktop)?;
        let encoder = Encoder::open(
            device.device(),
            &EncoderSettings {
                codec: encoder_codec_of(codec),
                width: size.0,
                height: size.1,
                fps: target.fps,
                bitrate_kbps,
                bit_depth: 8,
                preset: NATIVE_ENCODER_PRESET,
            },
        )
        .map_err(NativeError::Encoder)?;

        Ok(Self {
            encoder,
            converter,
            duplication: Some(duplication),
            device,
            hub,
            stop,
            codec,
            fps: target.fps,
            output,
            size,
            clock: FrameClock::new(target.fps),
            started: Instant::now(),
            has_picture: false,
            frames: 0,
        })
    }
}
