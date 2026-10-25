use super::{PictureOrderCountParts, ScalingListParts};

#[derive(Debug, Clone)]
pub struct SpsParts {
    pub profile_idc: u32,
    pub chroma_format_idc: u32,
    pub separate_colour_plane: bool,
    pub scaling_matrix: Option<Vec<ScalingListParts>>,
    pub picture_order_count: PictureOrderCountParts,
    pub width_in_macroblocks: u32,
    pub height_in_map_units: u32,
    pub frame_mbs_only: bool,
    pub crop: Option<[u32; 4]>,
}

impl Default for SpsParts {
    fn default() -> Self {
        Self {
            profile_idc: 100,
            chroma_format_idc: 1,
            separate_colour_plane: false,
            scaling_matrix: None,
            picture_order_count: PictureOrderCountParts::Type0,
            width_in_macroblocks: 120,
            height_in_map_units: 68,
            frame_mbs_only: true,
            crop: Some([0, 0, 0, 4]),
        }
    }
}
