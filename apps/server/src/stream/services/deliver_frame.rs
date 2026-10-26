use crate::hub::Frame;
use crate::stream::{Admission, SessionCursor, SessionSender};

pub fn deliver_frame(sender: &mut SessionSender, cursor: &mut SessionCursor, frame: &Frame) {
    match cursor.admit(frame) {
        Admission::Skip => {}
        Admission::Send => sender.send_video(frame),
        Admission::SendWithConfig(config) => {
            sender.send_config(&config);
            sender.send_video(frame);
        }
    }
}
