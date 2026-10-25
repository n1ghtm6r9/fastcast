use crate::web::{ViewerQuery, display_mode_from};

#[test]
fn a_mode_outside_the_supported_range_asks_for_no_mode() {
    let mode_of = |width: &str, height: &str, fps: &str| {
        display_mode_from(&ViewerQuery {
            width: Some(width.into()),
            height: Some(height.into()),
            fps: Some(fps.into()),
            codecs: None,
        })
    };

    assert_eq!(mode_of("100000", "2160", "60"), None);
    assert_eq!(mode_of("3840", "100", "60"), None);
    assert_eq!(mode_of("3840", "2160", "0"), None);
    assert_eq!(mode_of("3840", "2160", "1000"), None);
    assert_eq!(mode_of("wide", "2160", "60"), None);
}
