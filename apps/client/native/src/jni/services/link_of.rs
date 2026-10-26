use crate::jni::links;
use crate::link::Link;
use std::sync::{Arc, PoisonError};

pub fn link_of(id: i64) -> Option<Arc<Link>> {
    links()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&id)
        .cloned()
}
