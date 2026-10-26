use super::{GPU_SLOTS, HELD_GPU_SLOT, try_lock_gpu_slot};
use std::thread::sleep;
use std::time::Duration;

pub fn hold_gpu_slot() {
    HELD_GPU_SLOT.with(|held| {
        if held.borrow().is_some() {
            return;
        }

        let slot = loop {
            if let Some(slot) = (0..GPU_SLOTS).find_map(try_lock_gpu_slot) {
                break slot;
            }
            sleep(Duration::from_millis(20));
        };
        *held.borrow_mut() = Some(slot);
    });
}
