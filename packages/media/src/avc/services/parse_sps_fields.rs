use crate::avc::{
    BitReader, ChromaFormat, HIGH_PROFILE_IDCS, SpsFields, read_frame_crop,
    read_high_profile_fields, skip_picture_order_count,
};

pub fn parse_sps_fields(rbsp: &[u8]) -> Option<SpsFields> {
    let mut reader = BitReader::new(rbsp);
    let profile_idc = reader.read_bits(8)?;
    let _constraint_flags = reader.read_bits(8)?;
    let _level_idc = reader.read_bits(8)?;
    let _seq_parameter_set_id = reader.read_ue()?;

    let chroma_array_type = if HIGH_PROFILE_IDCS.contains(&profile_idc) {
        read_high_profile_fields(&mut reader)?
    } else {
        ChromaFormat::Yuv420
    };

    let _log2_max_frame_num_minus4 = reader.read_ue()?;
    skip_picture_order_count(&mut reader)?;
    let _max_num_ref_frames = reader.read_ue()?;
    let _gaps_in_frame_num_value_allowed_flag = reader.read_bit()?;
    let width_in_macroblocks = reader.read_ue()?.checked_add(1)?;
    let height_in_map_units = reader.read_ue()?.checked_add(1)?;
    let frame_mbs_only = reader.read_bit()? == 1;

    if !frame_mbs_only {
        let _mb_adaptive_frame_field_flag = reader.read_bit()?;
    }

    let _direct_8x8_inference_flag = reader.read_bit()?;
    let crop = read_frame_crop(&mut reader)?;

    Some(SpsFields {
        chroma_array_type,
        width_in_macroblocks,
        height_in_map_units,
        frame_mbs_only,
        crop,
    })
}
