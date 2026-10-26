use crate::avc::{
    AVCC_BIT_DEPTH_RESERVED_BITS, AVCC_CHROMA_FORMAT_RESERVED_BITS,
    AVCC_FOUR_BYTE_NAL_LENGTH_FIELD, AVCC_NO_SPS_EXTENSIONS, AVCC_ONE_PPS, AVCC_ONE_SPS,
    AVCC_VERSION, HIGH_PROFILE_IDCS, avc_sample_format, remove_emulation_prevention,
};

pub fn avcc_from_parameter_sets(sps: &[u8], pps: &[u8]) -> Option<Vec<u8>> {
    let [_, profile, compatibility, level, ..] = sps else {
        return None;
    };
    let sps_length = u16::try_from(sps.len()).ok()?;
    let pps_length = u16::try_from(pps.len()).ok()?;

    let mut record = vec![
        AVCC_VERSION,
        *profile,
        *compatibility,
        *level,
        AVCC_FOUR_BYTE_NAL_LENGTH_FIELD,
        AVCC_ONE_SPS,
    ];
    record.extend_from_slice(&sps_length.to_be_bytes());
    record.extend_from_slice(sps);
    record.push(AVCC_ONE_PPS);
    record.extend_from_slice(&pps_length.to_be_bytes());
    record.extend_from_slice(pps);

    if HIGH_PROFILE_IDCS.contains(&u32::from(*profile)) {
        let format = avc_sample_format(&remove_emulation_prevention(&sps[1..]))?;

        record.extend_from_slice(&[
            AVCC_CHROMA_FORMAT_RESERVED_BITS | format.chroma_format_idc,
            AVCC_BIT_DEPTH_RESERVED_BITS | format.bit_depth_luma_minus8,
            AVCC_BIT_DEPTH_RESERVED_BITS | format.bit_depth_chroma_minus8,
            AVCC_NO_SPS_EXTENSIONS,
        ]);
    }

    Some(record)
}
