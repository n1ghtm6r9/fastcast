use crate::support::FRAME_DATA;
use app::hub::Hub;

pub fn publish_frame(hub: &Hub, key: bool, timestamp: u32) {
    hub.publish(key, timestamp, &FRAME_DATA);
}
