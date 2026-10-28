pub fn share_changed(before: &[u16], after: &[u16]) -> f64 {
    let changed = before.iter().zip(after).filter(|(a, b)| a != b).count();

    changed as f64 / before.len() as f64
}
