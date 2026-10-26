use crate::hub::Hub;

pub fn publish_gop(hub: &Hub, deltas: usize) {
    hub.publish(true, 0, &[1; 3000]);

    for index in 0..deltas {
        hub.publish(false, 16 * (index as u32 + 1), &[2; 200]);
    }
}
