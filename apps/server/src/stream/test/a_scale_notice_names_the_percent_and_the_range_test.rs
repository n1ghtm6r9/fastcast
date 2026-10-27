use crate::display::DisplayScale;
use crate::stream::scale_notice_json;

#[test]
fn a_scale_notice_names_the_percent_and_the_range() {
    assert_eq!(
        scale_notice_json(&DisplayScale {
            percent: 175,
            min: 100,
            max: 300,
        }),
        r#"{"type":"scale","percent":175,"min":100,"max":300}"#
    );
}
