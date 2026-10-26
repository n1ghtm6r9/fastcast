use crate::capture::{CaptureError, EncoderPicker, VideoEncoder, pick_encoder};
use media::VideoCodec;
use std::future::Future;
use tracing::warn;

pub async fn pick_codec_and_encoder<'a, S, P, F>(
    codecs: &[VideoCodec],
    picker_for: S,
    probe: P,
) -> Result<(VideoEncoder, VideoCodec), CaptureError>
where
    S: Fn(VideoCodec) -> &'a EncoderPicker,
    P: Fn(VideoEncoder, VideoCodec) -> F,
    F: Future<Output = Result<bool, CaptureError>>,
{
    let mut tried = Vec::new();

    for &codec in codecs {
        match pick_encoder(picker_for(codec), |encoder| probe(encoder, codec)).await {
            Ok(encoder) => return Ok((encoder, codec)),
            Err(CaptureError::NoEncoderAvailable { tried: unusable }) => {
                warn!("no {codec:?} encoder works on this machine");

                for encoder in unusable {
                    if !tried.contains(&encoder) {
                        tried.push(encoder);
                    }
                }
            }
            Err(other) => return Err(other),
        }
    }

    Err(CaptureError::NoEncoderAvailable { tried })
}
