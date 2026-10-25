use crate::packet::test::raw_datagram;
use crate::receiver::FrameReceiver;
use crate::receiver::test::{deliveries, feed, ms};
use crate::test_support::{random_bytes, xorshift};
use std::time::Instant;

#[test]
fn random_shards_never_panic_the_receiver() {
    let mut state = 0xDEAD_BEEF_CAFE_F00D;
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();

    for index in 0..10_000u64 {
        let length = (xorshift(&mut state) % 1501) as usize;
        let payload = random_bytes(&mut state, length);
        let now = t0 + ms(index);

        feed(&mut receiver, &raw_datagram(3, &payload), now);
        deliveries(&mut receiver, now);
    }
}
