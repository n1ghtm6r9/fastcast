use crate::layout::{FrameLayout, MAX_FRAME_SIZE};
use crate::packet::{Datagram, Packet, Shard, ShardHeader, write_datagram};
use crate::packetizer::{
    Datagrams, OutgoingFrame, Packetizer, encode_blocks, next_counter, plan_blocks, send_order,
    shard_data,
};

impl Packetizer {
    pub fn frame(
        &mut self,
        frame: &OutgoingFrame<'_>,
        fec_percent: u8,
        out: &mut Datagrams,
    ) -> u32 {
        out.clear();

        let frame_number = next_counter(&mut self.next_frame);

        if frame.payload.len() > MAX_FRAME_SIZE {
            return frame_number;
        }

        let layout = FrameLayout::new(frame.payload.len() as u32);
        let scratch_length = plan_blocks(&layout, fec_percent, &mut self.plans);

        self.blocks.clear();
        self.blocks.resize(scratch_length, 0);
        encode_blocks(frame.payload, &layout, &self.plans, &mut self.blocks);

        for (block, index) in send_order(&self.plans) {
            let plan = &self.plans[block];
            let header = ShardHeader {
                frame: frame_number,
                frame_kind: frame.kind,
                keyframe: frame.keyframe,
                block: block as u16,
                blocks: layout.blocks as u16,
                data_shards: plan.data_shards as u8,
                parity_shards: plan.parity_shards as u8,
                shard: index as u8,
                frame_size: layout.frame_size,
            };
            let data = shard_data(frame.payload, &layout, plan, index, &self.blocks);
            let datagram = Datagram {
                session: self.session,
                sequence: next_counter(&mut self.next_sequence),
                packet: Packet::Shard(Shard { header, data }),
            };

            out.push_with(|bytes| write_datagram(&datagram, bytes));
        }

        frame_number
    }
}
