use crate::web::{ViewerQuery, display_mode_from};

#[test]
fn a_query_missing_part_of_the_mode_asks_for_no_mode() {
    let query = ViewerQuery {
        width: Some("3840".into()),
        height: Some("2160".into()),
        ..ViewerQuery::default()
    };

    assert_eq!(display_mode_from(&query), None);
}
