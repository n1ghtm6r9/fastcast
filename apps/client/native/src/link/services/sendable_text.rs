use crate::link::{LinkShared, TEXT_LIMIT};
use std::sync::atomic::Ordering;

pub(crate) fn sendable_text<'a>(shared: &LinkShared, text: &'a [u8]) -> Option<&'a str> {
    if text.len() > TEXT_LIMIT || shared.session.load(Ordering::Acquire) == 0 {
        return None;
    }

    str::from_utf8(text).ok()
}
