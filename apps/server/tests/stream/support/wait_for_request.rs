use crate::support::{ARRIVAL_LIMIT, wait_until};
use app::hub::{Hub, ViewerRequest};

pub fn wait_for_request(hub: &Hub, request: &ViewerRequest) {
    let requests = hub.watch_requests();

    assert!(
        wait_until(ARRIVAL_LIMIT, || *requests.borrow() == *request),
        "the request {request:?} arrives in time"
    );
}
