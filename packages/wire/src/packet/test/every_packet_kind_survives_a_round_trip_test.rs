use crate::packet::{
    Codec, CodecSupport, Datagram, Feedback, FrameKind, Hello, Packet, Shard, ShardHeader, Welcome,
    decode_datagram, encode_datagram,
};

#[test]
fn every_packet_kind_survives_a_round_trip() {
    let header = ShardHeader {
        frame: 11,
        frame_kind: FrameKind::Video,
        keyframe: false,
        block: 3,
        blocks: 4,
        data_shards: 100,
        parity_shards: 10,
        shard: 105,
        frame_size: 123_456,
    };
    let packets = [
        Packet::Hello(Hello {
            nonce: 0xDEAD_BEEF,
            width: 3840,
            height: 2160,
            fps: 120,
            codecs: vec![
                CodecSupport {
                    codec: Codec::Hevc,
                    max_bit_depth: 10,
                },
                CodecSupport {
                    codec: Codec::Avc,
                    max_bit_depth: 8,
                },
            ],
        }),
        Packet::Welcome(Welcome {
            nonce: 5,
            shard_size: 1370,
        }),
        Packet::Shard(Shard {
            header,
            data: &[1, 2, 3],
        }),
        Packet::Feedback(Feedback {
            highest_frame: 101,
            received_packets: 202,
            highest_sequence: 303,
            lost_frames: 404,
            recovered_shards: 505,
            keyframe_needed: true,
        }),
        Packet::Bye,
        Packet::Stats("{\"fps\":120}"),
    ];

    for (index, packet) in packets.into_iter().enumerate() {
        let datagram = Datagram {
            session: 0x1234_5678,
            sequence: index as u32,
            packet,
        };
        let mut out = Vec::new();

        encode_datagram(&datagram, &mut out);

        assert_eq!(decode_datagram(&out), Some(datagram));
    }
}
