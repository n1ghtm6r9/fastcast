use crate::capture::{
    CaptureTarget, FrameClock, KeyFrameGate, NativeCapture, NativeError, NativePipeline, Picture,
    ReopenFailures, converter_depth_of, encoder_codec_of, encoder_preset_for, nvidia_output_for,
};
use crate::hub::Hub;
use desktop::{Converter, Duplication, GpuDevice, list_outputs};
use nvenc::{Encoder, EncoderSettings};
use std::sync::atomic::AtomicBool;
use std::time::Instant;
use tracing::info;

impl<'a> NativeCapture<'a> {
    pub fn open(
        hub: &'a Hub,
        pipeline: NativePipeline,
        target: &CaptureTarget,
        bitrate_kbps: u32,
        stop: &'a AtomicBool,
    ) -> Result<Self, NativeError> {
        let outputs = list_outputs().map_err(NativeError::Desktop)?;
        let output = nvidia_output_for(target, &outputs)?.clone();
        let device = GpuDevice::new(output.adapter_luid).map_err(NativeError::Desktop)?;
        let duplication = Duplication::new(&device, &output).map_err(NativeError::Desktop)?;
        let size = (duplication.width(), duplication.height());
        let converter = Converter::new(
            &device,
            size.0,
            size.1,
            converter_depth_of(pipeline.bit_depth),
        )
        .map_err(NativeError::Desktop)?;
        let preset = encoder_preset_for(size.0, size.1, target.fps);

        info!(
            "encoding {}x{} {}-bit at {} fps with NVENC preset P{}",
            size.0, size.1, pipeline.bit_depth, target.fps, preset
        );

        let encoder = Encoder::open(
            device.device(),
            &EncoderSettings {
                codec: encoder_codec_of(pipeline.codec),
                width: size.0,
                height: size.1,
                fps: target.fps,
                bitrate_kbps,
                bit_depth: pipeline.bit_depth,
                preset,
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
            codec: pipeline.codec,
            fps: target.fps,
            output,
            size,
            clock: FrameClock::new(target.fps),
            key_frames: KeyFrameGate::default(),
            reopen_failures: ReopenFailures::default(),
            started: Instant::now(),
            picture: Picture::default(),
            frames: 0,
            pending_shown: None,
        })
    }
}
