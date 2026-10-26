use crate::stream::SESSION_COUNTER;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::sync::atomic::Ordering;
use std::time::Instant;

pub fn new_session_id(avoid: u32) -> u32 {
    loop {
        let mut hasher = RandomState::new().build_hasher();

        Instant::now().hash(&mut hasher);
        SESSION_COUNTER
            .fetch_add(1, Ordering::Relaxed)
            .hash(&mut hasher);

        let id = hasher.finish() as u32;

        if id != 0 && id != avoid {
            return id;
        }
    }
}
