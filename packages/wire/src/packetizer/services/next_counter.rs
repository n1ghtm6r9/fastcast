pub fn next_counter(counter: &mut u32) -> u32 {
    let value = *counter;

    *counter = value.wrapping_add(1);

    value
}
