use crate::jni::links;
use crate::link::Link;
use std::sync::{Arc, PoisonError};

pub(crate) fn forget_link(id: i64) -> Option<Arc<Link>> {
    links()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .remove(&id)
}
