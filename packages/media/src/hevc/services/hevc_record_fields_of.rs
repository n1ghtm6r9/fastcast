use crate::avc::{BitReader, ChromaFormat, SampleFormat, read_frame_crop};
use crate::hevc::{
    HEVC_PROFILE_TIER_LEVEL_LENGTH, HEVC_PROFILE_TIER_LEVEL_OFFSET, HevcRecordFields,
    skip_profile_tier_level,
};

pub fn hevc_record_fields_of(rbsp: &[u8]) -> Option<HevcRecordFields> {
    let general_profile_tier_level = <[u8; HEVC_PROFILE_TIER_LEVEL_LENGTH]>::try_from(rbsp.get(
        HEVC_PROFILE_TIER_LEVEL_OFFSET
            ..HEVC_PROFILE_TIER_LEVEL_OFFSET + HEVC_PROFILE_TIER_LEVEL_LENGTH,
    )?)
    .ok()?;

    let mut reader = BitReader::new(rbsp);
    let _sps_video_parameter_set_id = reader.read_bits(4)?;
    let max_sub_layers_minus1 = reader.read_bits(3)?;
    let temporal_id_nesting = reader.read_bit()? == 1;
    skip_profile_tier_level(&mut reader, max_sub_layers_minus1)?;
    let _sps_seq_parameter_set_id = reader.read_ue()?;
    let chroma_format_idc = reader.read_ue()?;
    let chroma_format = ChromaFormat::from_idc(chroma_format_idc)?;

    if chroma_format == ChromaFormat::Yuv444 {
        let _separate_colour_plane_flag = reader.read_bit()?;
    }

    let _width_in_luma_samples = reader.read_ue()?;
    let _height_in_luma_samples = reader.read_ue()?;
    let _conformance_window = read_frame_crop(&mut reader)?;
    let bit_depth_luma_minus8 = reader.read_ue()?;
    let bit_depth_chroma_minus8 = reader.read_ue()?;

    Some(HevcRecordFields {
        general_profile_tier_level,
        max_sub_layers_minus1: u8::try_from(max_sub_layers_minus1).ok()?,
        temporal_id_nesting,
        sample_format: SampleFormat {
            chroma_format_idc: u8::try_from(chroma_format_idc).ok()?,
            bit_depth_luma_minus8: u8::try_from(bit_depth_luma_minus8).ok()?,
            bit_depth_chroma_minus8: u8::try_from(bit_depth_chroma_minus8).ok()?,
        },
    })
}
