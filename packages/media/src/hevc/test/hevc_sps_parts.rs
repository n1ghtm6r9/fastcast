use super::SubLayerParts;

#[derive(Debug, Clone)]
pub struct HevcSpsParts {
    pub sub_layers: Vec<SubLayerParts>,
    pub chroma_format_idc: u32,
    pub separate_colour_plane: bool,
    pub width_in_luma_samples: u32,
    pub height_in_luma_samples: u32,
    pub conformance_window: Option<[u32; 4]>,
}

impl Default for HevcSpsParts {
    fn default() -> Self {
        Self {
            sub_layers: Vec::new(),
            chroma_format_idc: 1,
            separate_colour_plane: false,
            width_in_luma_samples: 1920,
            height_in_luma_samples: 1088,
            conformance_window: Some([0, 0, 0, 4]),
        }
    }
}
