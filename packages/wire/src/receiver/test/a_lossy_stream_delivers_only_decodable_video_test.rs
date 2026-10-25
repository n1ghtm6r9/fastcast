use crate::packet::FrameKind;
use crate::packetizer::Packetizer;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use crate::receiver::{FrameReceiver, ReceivedFrame};
use crate::test_support::{random_bytes, xorshift};
use std::time::{Duration, Instant};

#[test]
fn a_lossy_stream_delivers_only_decodable_video() {
    let mut state = 0x1234_5678_9ABC_DEF1;
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    let mut sent: Vec<Vec<u8>> = Vec::new();
    let mut delivered: Vec<ReceivedFrame> = Vec::new();

    for number in 0..300u32 {
        let keyframe = number % 30 == 0;
        let length = if keyframe { 200_000 } else { 20_000 };
        let payload = random_bytes(&mut state, length);
        let now = t0 + ms(8 * u64::from(number));

        for datagram in datagrams_of(&mut packetizer, FrameKind::Video, keyframe, &payload, 10) {
            if xorshift(&mut state) % 100 >= 3 {
                feed(&mut receiver, &datagram, now);
            }
        }

        sent.push(payload);
        delivered.extend(deliveries(&mut receiver, now));
    }
    delivered.extend(deliveries(&mut receiver, t0 + Duration::from_secs(10)));

    assert!(delivered.len() > 100);
    assert!(receiver.recovered_shards() > 0);
    assert!(receiver.lost_frames() > 0);
    assert!(
        delivered
            .windows(2)
            .all(|pair| pair[0].number < pair[1].number)
    );
    assert!(
        delivered
            .iter()
            .all(|frame| frame.payload == sent[frame.number as usize])
    );
    for (index, frame) in delivered.iter().enumerate() {
        if !frame.keyframe {
            assert!(index > 0 && frame.number == delivered[index - 1].number + 1);
        }
    }
}
