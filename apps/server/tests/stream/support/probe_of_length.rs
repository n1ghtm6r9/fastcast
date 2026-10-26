pub fn probe_of_length(length: usize) -> Vec<u8> {
    let mut bytes = vec![b' '; length];

    bytes[0] = b'{';

    bytes
}
