use crate::avc::{BitReader, ChromaFormat, HIGH_PROFILE_IDCS, SampleFormat};

pub fn avc_sample_format(rbsp: &[u8]) -> Option<SampleFormat> {
    let mut reader = BitReader::new(rbsp);
    let profile_idc = reader.read_bits(8)?;
    let _constraint_flags_and_level_idc = reader.read_bits(16)?;
    let _seq_parameter_set_id = reader.read_ue()?;

    if !HIGH_PROFILE_IDCS.contains(&profile_idc) {
        return Some(SampleFormat {
            chroma_format_idc: 1,
            bit_depth_luma_minus8: 0,
            bit_depth_chroma_minus8: 0,
        });
    }

    let chroma_format_idc = reader.read_ue()?;
    let chroma_format = ChromaFormat::from_idc(chroma_format_idc)?;

    if chroma_format == ChromaFormat::Yuv444 {
        let _separate_colour_plane_flag = reader.read_bit()?;
    }

    let bit_depth_luma_minus8 = reader.read_ue()?;
    let bit_depth_chroma_minus8 = reader.read_ue()?;

    Some(SampleFormat {
        chroma_format_idc: u8::try_from(chroma_format_idc).ok()?,
        bit_depth_luma_minus8: u8::try_from(bit_depth_luma_minus8).ok()?,
        bit_depth_chroma_minus8: u8::try_from(bit_depth_chroma_minus8).ok()?,
    })
}
