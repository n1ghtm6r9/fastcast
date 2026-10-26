use crate::support::{ARRIVAL_LIMIT, TestViewer};
use std::time::Instant;
use wire::{FrameKind, Packet, ShardHeader, decode_datagram};

pub fn next_video_shard_header_after(viewer: &TestViewer, newest: Option<u32>) -> ShardHeader {
    let deadline = Instant::now() + ARRIVAL_LIMIT;

    loop {
        let bytes = viewer
            .next_datagram(deadline.saturating_duration_since(Instant::now()))
            .expect("a video shard of a newer frame arrives in time");

        if let Some(datagram) = decode_datagram(&bytes)
            && let Packet::Shard(shard) = datagram.packet
            && shard.header.frame_kind == FrameKind::Video
            && newest.is_none_or(|newest| shard.header.frame > newest)
        {
            return shard.header;
        }
    }
}
