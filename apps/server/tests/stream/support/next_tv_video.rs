use crate::support::{VideoFrame, video_frame_of};
use fastcast_native::{Link, LinkEvent};
use std::time::Duration;

pub fn next_tv_video(link: &Link, limit: Duration) -> Option<VideoFrame> {
    loop {
        match link.next_event(limit)? {
            LinkEvent::Video(payload) => return Some(video_frame_of(&payload)),
            LinkEvent::Config(_) => continue,
            other => panic!("the tv link expected video, it got {other:?}"),
        }
    }
}
