use crate::jni::{links, next_link_id};
use crate::link::Link;
use std::sync::{Arc, PoisonError};

pub(crate) fn register_link(link: Link) -> i64 {
    let id = next_link_id();

    links()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(id, Arc::new(link));

    id
}
