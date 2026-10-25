use crate::hub::ViewerRequest;
use crate::web::{ViewerQuery, viewer_request_from};

#[test]
fn a_bare_connection_asks_for_nothing() {
    assert_eq!(
        viewer_request_from(&ViewerQuery::default()),
        ViewerRequest::default()
    );
}
