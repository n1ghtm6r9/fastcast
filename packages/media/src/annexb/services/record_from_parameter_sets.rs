use crate::VideoCodec;
use crate::avc::avcc_from_parameter_sets;
use crate::hevc::hvcc_from_parameter_sets;

pub fn record_from_parameter_sets(
    codec: VideoCodec,
    vps: Option<&[u8]>,
    sps: Option<&[u8]>,
    pps: Option<&[u8]>,
) -> Option<Vec<u8>> {
    match codec {
        VideoCodec::Avc => avcc_from_parameter_sets(sps?, pps?),
        VideoCodec::Hevc => hvcc_from_parameter_sets(vps?, sps?, pps?),
    }
}
