use crate::capture::{
    CaptureStats, CaptureTarget, FrameClock, KeyFrameGate, NativeCapture, NativeError,
    NativePipeline, Picture, ReopenFailures, converter_depth_of, encoder_settings_for,
    nvidia_output_for, raise_capture_priorities,
};
use crate::hub::Hub;
use desktop::{Converter, Duplication, GpuDevice, NoisePainter, list_outputs};
use nvenc::Encoder;
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
        info!("{}", raise_capture_priorities(&device));
        let duplication = Duplication::new(&device, &output).map_err(NativeError::Desktop)?;
        let size = (duplication.width(), duplication.height());
        let converter = Converter::new(
            &device,
            size.0,
            size.1,
            converter_depth_of(pipeline.bit_depth),
        )
        .map_err(NativeError::Desktop)?;
        let settings = encoder_settings_for(pipeline, size, target.fps, bitrate_kbps);

        info!(
            "encoding {}x{} {}-bit at {} fps with NVENC preset P{}",
            size.0, size.1, pipeline.bit_depth, target.fps, settings.preset
        );

        let encoder = Encoder::open(device.device(), &settings).map_err(NativeError::Encoder)?;
        let noise = pipeline
            .noise
            .then(|| NoisePainter::new(&device))
            .transpose()
            .map_err(NativeError::Desktop)?;

        if noise.is_some() {
            info!("the TV asked for noise: every frame is new random pixels instead of the screen");
        }

        Ok(Self {
            encoder,
            converter,
            noise,
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
            pending_arrived: None,
            stats: CaptureStats::new(Instant::now()),
        })
    }
}
