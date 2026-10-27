use wire::Feedback;

pub fn link_line(first: &Feedback, last: &Feedback, keyframe_asks: u32, fec_percent: u8) -> String {
    let sent = last.highest_sequence.wrapping_sub(first.highest_sequence);
    let received = last.received_packets.wrapping_sub(first.received_packets);
    let lost_percent = if sent == 0 {
        0.0
    } else {
        f64::from(sent.saturating_sub(received)) * 100.0 / f64::from(sent)
    };
    let lost_frames = last.lost_frames.wrapping_sub(first.lost_frames);
    let rebuilt_shards = last.recovered_shards.wrapping_sub(first.recovered_shards);

    format!(
        "{received} of {sent} packets arrived ({lost_percent:.1}% lost), {lost_frames} frames lost, \
         {rebuilt_shards} shards rebuilt, {keyframe_asks} keyframes asked, FEC {fec_percent}%"
    )
}
