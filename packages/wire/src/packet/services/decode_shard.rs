use crate::packet::{
    ByteReader, KEYFRAME_FLAG, SHARD_HEADER_SIZE, Shard, ShardHeader, frame_kind_from_byte,
};

pub fn decode_shard<'a>(reader: &mut ByteReader<'a>) -> Option<Shard<'a>> {
    let mut fields = ByteReader::new(reader.take(SHARD_HEADER_SIZE)?);
    let frame = fields.read_u32()?;
    let frame_kind = frame_kind_from_byte(fields.read_u8()?)?;
    let flags = fields.read_u8()?;
    let block = fields.read_u16()?;
    let blocks = fields.read_u16()?;
    let data_shards = fields.read_u8()?;
    let parity_shards = fields.read_u8()?;
    let shard = fields.read_u8()?;
    let _reserved = fields.read_u8()?;
    let frame_size = fields.read_u32()?;

    Some(Shard {
        header: ShardHeader {
            frame,
            frame_kind,
            keyframe: flags & KEYFRAME_FLAG != 0,
            block,
            blocks,
            data_shards,
            parity_shards,
            shard,
            frame_size,
        },
        data: reader.rest(),
    })
}
