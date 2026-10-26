use crate::av1::{
    AV1C_MARKER_AND_VERSION, AV1C_NO_PRESENTATION_DELAY, AV1C_PROFILE_SHIFT, Obu, av1c_flags_of,
    parse_av1_sequence_header,
};

pub fn av1c_from_sequence_header(sequence_header: &Obu) -> Option<Vec<u8>> {
    let header = parse_av1_sequence_header(sequence_header.payload)?;

    let mut av1c = vec![
        AV1C_MARKER_AND_VERSION,
        (header.profile << AV1C_PROFILE_SHIFT) | header.level,
        av1c_flags_of(&header),
        AV1C_NO_PRESENTATION_DELAY,
    ];

    av1c.extend_from_slice(sequence_header.bytes);

    Some(av1c)
}
