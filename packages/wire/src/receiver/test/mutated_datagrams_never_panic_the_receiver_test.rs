use crate::packet::{FrameKind, HEADER_SIZE, SHARD_HEADER_SIZE};
use crate::packetizer::Packetizer;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{datagrams_of, deliveries, feed, ms};
use crate::test_support::{random_bytes, xorshift};
use std::time::Instant;

#[test]
fn mutated_datagrams_never_panic_the_receiver() {
    let mut state = 0x0123_4567_89AB_CDEF;
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for number in 0..200u32 {
        let length = (xorshift(&mut state) % 40_000) as usize;
        let payload = random_bytes(&mut state, length);
        let now = t0 + ms(8 * u64::from(number));

        for mut datagram in datagrams_of(
            &mut packetizer,
            FrameKind::Video,
            number % 10 == 0,
            &payload,
            20,
        ) {
            for _ in 0..xorshift(&mut state) % 4 {
                let position = HEADER_SIZE + xorshift(&mut state) as usize % SHARD_HEADER_SIZE;
                datagram[position] = (xorshift(&mut state) >> 24) as u8;
            }

            feed(&mut receiver, &datagram, now);
        }

        deliveries(&mut receiver, now);
    }
}
