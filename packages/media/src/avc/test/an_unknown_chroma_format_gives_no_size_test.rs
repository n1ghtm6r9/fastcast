use super::{SpsParts, video_size_of_parts};

#[test]
fn an_unknown_chroma_format_gives_no_size() {
    let parts = SpsParts {
        chroma_format_idc: 4,
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), None);
}
