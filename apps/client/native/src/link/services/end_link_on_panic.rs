use crate::link::{LinkShared, RECEIVER_FAILED, end_link};
use std::panic::{AssertUnwindSafe, catch_unwind};

pub(crate) fn end_link_on_panic(shared: &LinkShared, body: impl FnOnce()) {
    if catch_unwind(AssertUnwindSafe(body)).is_err() {
        end_link(shared, RECEIVER_FAILED);
    }
}
