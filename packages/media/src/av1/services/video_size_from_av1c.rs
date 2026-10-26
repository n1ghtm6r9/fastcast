use crate::av1::{AV1C_HEADER_LENGTH, OBU_SEQUENCE_HEADER, obus_of, parse_av1_sequence_header};
use crate::avc::VideoSize;

pub fn video_size_from_av1c(av1c: &[u8]) -> Option<VideoSize> {
    let config_obus = av1c.get(AV1C_HEADER_LENGTH..)?;

    let sequence_header = obus_of(config_obus)
        .into_iter()
        .find(|obu| obu.kind == OBU_SEQUENCE_HEADER)?;

    let header = parse_av1_sequence_header(sequence_header.payload)?;

    Some(VideoSize {
        width: header.max_frame_width,
        height: header.max_frame_height,
    })
}
