use crate::avc::{
    BitReader, ChromaFormat, SCALING_LISTS_WITH_SEPARATE_CHROMA,
    SCALING_LISTS_WITHOUT_SEPARATE_CHROMA, skip_scaling_matrix,
};

pub fn read_high_profile_fields(reader: &mut BitReader) -> Option<ChromaFormat> {
    let chroma_format = ChromaFormat::from_idc(reader.read_ue()?)?;
    let has_three_colour_planes = chroma_format == ChromaFormat::Yuv444;
    let separate_colour_plane = has_three_colour_planes && reader.read_bit()? == 1;
    let _bit_depth_luma_minus8 = reader.read_ue()?;
    let _bit_depth_chroma_minus8 = reader.read_ue()?;
    let _qpprime_y_zero_transform_bypass_flag = reader.read_bit()?;
    let scaling_matrix_present = reader.read_bit()? == 1;

    if scaling_matrix_present {
        let list_count = if has_three_colour_planes {
            SCALING_LISTS_WITH_SEPARATE_CHROMA
        } else {
            SCALING_LISTS_WITHOUT_SEPARATE_CHROMA
        };

        skip_scaling_matrix(reader, list_count)?;
    }

    if separate_colour_plane {
        Some(ChromaFormat::Monochrome)
    } else {
        Some(chroma_format)
    }
}
