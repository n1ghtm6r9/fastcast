use crate::packet::{KEYFRAME_FLAG, Shard};

pub fn write_shard(shard: &Shard<'_>, out: &mut Vec<u8>) {
    let header = &shard.header;

    out.extend_from_slice(&header.frame.to_le_bytes());
    out.push(header.frame_kind as u8);
    out.push(if header.keyframe { KEYFRAME_FLAG } else { 0 });
    out.extend_from_slice(&header.block.to_le_bytes());
    out.extend_from_slice(&header.blocks.to_le_bytes());
    out.push(header.data_shards);
    out.push(header.parity_shards);
    out.push(header.shard);
    out.push(0);
    out.extend_from_slice(&header.frame_size.to_le_bytes());
    out.extend_from_slice(shard.data);
}
