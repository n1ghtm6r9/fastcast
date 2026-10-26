use crate::capture::spawn_capture_worker;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn a_worker_starts_only_after_the_previous_one_has_finished() {
    let order = Arc::new(Mutex::new(Vec::new()));

    let first = spawn_capture_worker(None, {
        let order = order.clone();
        move || {
            thread::sleep(Duration::from_millis(100));
            order.lock().unwrap().push("first finished");
        }
    })
    .expect("the first worker starts");
    let second = spawn_capture_worker(Some(first), {
        let order = order.clone();
        move || order.lock().unwrap().push("second started")
    })
    .expect("the second worker starts");

    second.join().expect("the second worker ends");

    assert_eq!(
        *order.lock().unwrap(),
        vec!["first finished", "second started"]
    );
}
