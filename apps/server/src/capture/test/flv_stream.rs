pub fn flv_stream(tags: &[Vec<u8>]) -> Vec<u8> {
    let mut stream = vec![b'F', b'L', b'V', 1, 1, 0, 0, 0, 9, 0, 0, 0, 0];

    for tag in tags {
        stream.extend_from_slice(tag);
    }

    stream
}
