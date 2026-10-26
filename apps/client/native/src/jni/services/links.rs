use crate::link::Link;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub fn links() -> &'static Mutex<HashMap<i64, Arc<Link>>> {
    static LINKS: OnceLock<Mutex<HashMap<i64, Arc<Link>>>> = OnceLock::new();

    LINKS.get_or_init(|| Mutex::new(HashMap::new()))
}
