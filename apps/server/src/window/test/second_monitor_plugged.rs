use std::cell::Cell;

thread_local! {
    pub static SECOND_MONITOR_PLUGGED: Cell<bool> = const { Cell::new(false) };
}
