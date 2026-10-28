use crate::pacing::{PACE_BURST_PACKETS, PACE_SPREAD_DIVISOR, PACE_TICK};
use std::time::Duration;

pub fn burst_sizes(packets: usize, frame_interval: Duration) -> Vec<usize> {
    if packets == 0 {
        return Vec::new();
    }

    let ticks = (frame_interval / PACE_SPREAD_DIVISOR).as_nanos() / PACE_TICK.as_nanos();
    let bursts = usize::try_from(ticks)
        .unwrap_or(usize::MAX)
        .min(packets.div_ceil(PACE_BURST_PACKETS))
        .max(1);
    let base = packets / bursts;
    let remainder = packets % bursts;

    (0..bursts)
        .map(|burst| base + usize::from(burst < remainder))
        .collect()
}
