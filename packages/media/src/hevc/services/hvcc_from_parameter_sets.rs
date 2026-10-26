use crate::avc::remove_emulation_prevention;
use crate::hevc::{
    HEVC_NAL_HEADER_LENGTH, HEVC_PPS_NAL_UNIT_TYPE, HEVC_SPS_NAL_UNIT_TYPE, HEVC_VPS_NAL_UNIT_TYPE,
    HVCC_BIT_DEPTH_RESERVED_BITS, HVCC_CHROMA_FORMAT_RESERVED_BITS,
    HVCC_FOUR_BYTE_NAL_LENGTH_FIELD, HVCC_NO_PARALLELISM, HVCC_NO_SPATIAL_SEGMENTATION,
    HVCC_ONE_UNIT_PER_ARRAY, HVCC_PARAMETER_SET_ARRAY_COUNT, HVCC_TEMPORAL_ID_NESTING_SHIFT,
    HVCC_TEMPORAL_LAYERS_SHIFT, HVCC_UNSPECIFIED_FRAME_RATE, HVCC_VERSION, hevc_record_fields_of,
};

pub fn hvcc_from_parameter_sets(vps: &[u8], sps: &[u8], pps: &[u8]) -> Option<Vec<u8>> {
    let sps_payload = sps.get(HEVC_NAL_HEADER_LENGTH..)?;
    let fields = hevc_record_fields_of(&remove_emulation_prevention(sps_payload))?;
    let temporal_layers = fields.max_sub_layers_minus1 + 1;

    let mut record = vec![HVCC_VERSION];
    record.extend_from_slice(&fields.general_profile_tier_level);
    record.extend_from_slice(&HVCC_NO_SPATIAL_SEGMENTATION);
    record.push(HVCC_NO_PARALLELISM);
    record.push(HVCC_CHROMA_FORMAT_RESERVED_BITS | fields.sample_format.chroma_format_idc);
    record.push(HVCC_BIT_DEPTH_RESERVED_BITS | fields.sample_format.bit_depth_luma_minus8);
    record.push(HVCC_BIT_DEPTH_RESERVED_BITS | fields.sample_format.bit_depth_chroma_minus8);
    record.extend_from_slice(&HVCC_UNSPECIFIED_FRAME_RATE);
    record.push(
        (temporal_layers << HVCC_TEMPORAL_LAYERS_SHIFT)
            | (u8::from(fields.temporal_id_nesting) << HVCC_TEMPORAL_ID_NESTING_SHIFT)
            | HVCC_FOUR_BYTE_NAL_LENGTH_FIELD,
    );
    record.push(HVCC_PARAMETER_SET_ARRAY_COUNT);

    for (nal_unit_type, unit) in [
        (HEVC_VPS_NAL_UNIT_TYPE, vps),
        (HEVC_SPS_NAL_UNIT_TYPE, sps),
        (HEVC_PPS_NAL_UNIT_TYPE, pps),
    ] {
        let length = u16::try_from(unit.len()).ok()?;

        record.push(nal_unit_type);
        record.extend_from_slice(&HVCC_ONE_UNIT_PER_ARRAY.to_be_bytes());
        record.extend_from_slice(&length.to_be_bytes());
        record.extend_from_slice(unit);
    }

    Some(record)
}
