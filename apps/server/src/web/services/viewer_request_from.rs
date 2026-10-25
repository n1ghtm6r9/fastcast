use crate::hub::ViewerRequest;
use crate::web::{ViewerQuery, codecs_from, display_mode_from};

pub fn viewer_request_from(query: &ViewerQuery) -> ViewerRequest {
    ViewerRequest {
        mode: display_mode_from(query),
        codecs: codecs_from(query.codecs.as_deref()),
    }
}
