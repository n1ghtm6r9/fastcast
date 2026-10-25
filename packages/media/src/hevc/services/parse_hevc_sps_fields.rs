use crate::avc::{BitReader, ChromaFormat, read_frame_crop};
use crate::hevc::{HevcSpsFields, skip_profile_tier_level};

pub fn parse_hevc_sps_fields(rbsp: &[u8]) -> Option<HevcSpsFields> {
    let mut reader = BitReader::new(rbsp);
    let _sps_video_parameter_set_id = reader.read_bits(4)?;
    let max_sub_layers_minus1 = reader.read_bits(3)?;
    let _sps_temporal_id_nesting_flag = reader.read_bit()?;
    skip_profile_tier_level(&mut reader, max_sub_layers_minus1)?;
    let _sps_seq_parameter_set_id = reader.read_ue()?;
    let chroma_format = ChromaFormat::from_idc(reader.read_ue()?)?;
    let separate_colour_plane = chroma_format == ChromaFormat::Yuv444 && reader.read_bit()? == 1;
    let width_in_luma_samples = reader.read_ue()?;
    let height_in_luma_samples = reader.read_ue()?;
    let conformance_window = read_frame_crop(&mut reader)?;

    let chroma_array_type = if separate_colour_plane {
        ChromaFormat::Monochrome
    } else {
        chroma_format
    };

    Some(HevcSpsFields {
        chroma_array_type,
        width_in_luma_samples,
        height_in_luma_samples,
        conformance_window,
    })
}
