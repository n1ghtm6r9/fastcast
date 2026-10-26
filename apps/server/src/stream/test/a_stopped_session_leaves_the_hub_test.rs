use super::{hub_with_x264_config, start_session, wait_for_viewer_count};
use crate::hub::ViewerRequest;

#[test]
fn a_stopped_session_leaves_the_hub() {
    let hub = hub_with_x264_config();
    let session = start_session(&hub, ViewerRequest::default());

    assert!(wait_for_viewer_count(&hub, 1));

    session.stop_and_join();

    assert_eq!(hub.viewer_count(), 0);
}
