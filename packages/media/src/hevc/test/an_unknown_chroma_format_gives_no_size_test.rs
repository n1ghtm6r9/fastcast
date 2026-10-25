use super::{HevcSpsParts, video_size_of_hevc_parts};

#[test]
fn an_unknown_chroma_format_gives_no_size() {
    let parts = HevcSpsParts {
        chroma_format_idc: 4,
        ..HevcSpsParts::default()
    };

    assert_eq!(video_size_of_hevc_parts(&parts), None);
}
