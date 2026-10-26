#[derive(Debug, Clone)]
pub struct Av1SequenceHeaderParts {
    pub profile: u32,
    pub reduced_still_picture_header: bool,
    pub timing_info: Option<bool>,
    pub buffer_delay_length: Option<u32>,
    pub initial_display_delay: bool,
    pub operating_points: Vec<(u32, bool)>,
    pub width: u32,
    pub height: u32,
    pub frame_id_numbers: bool,
    pub order_hint: bool,
    pub choose_screen_content_tools: bool,
    pub force_screen_content_tools: bool,
    pub choose_integer_mv: bool,
    pub high_bitdepth: bool,
    pub twelve_bit: bool,
    pub monochrome: bool,
    pub color_description: Option<[u32; 3]>,
    pub twelve_bit_subsampling: (bool, bool),
    pub chroma_sample_position: u32,
}

impl Default for Av1SequenceHeaderParts {
    fn default() -> Self {
        Self {
            profile: 0,
            reduced_still_picture_header: false,
            timing_info: None,
            buffer_delay_length: None,
            initial_display_delay: false,
            operating_points: vec![(8, false)],
            width: 1920,
            height: 1080,
            frame_id_numbers: false,
            order_hint: false,
            choose_screen_content_tools: true,
            force_screen_content_tools: false,
            choose_integer_mv: true,
            high_bitdepth: false,
            twelve_bit: false,
            monochrome: false,
            color_description: Some([1, 1, 1]),
            twelve_bit_subsampling: (true, true),
            chroma_sample_position: 1,
        }
    }
}
