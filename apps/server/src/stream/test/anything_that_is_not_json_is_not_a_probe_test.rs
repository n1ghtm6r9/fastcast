use crate::stream::is_discovery_probe;

#[test]
fn anything_that_is_not_json_is_not_a_probe() {
    let datagrams: [&[u8]; 8] = [
        b"",
        b"discover",
        b"hello",
        b"{",
        br#"{"type":"discover""#,
        br#"{"type":"discover"} and more"#,
        b"\xff\xfe\x00\x9f",
        br#"{'type':'discover'}"#,
    ];

    for datagram in datagrams {
        assert!(
            !is_discovery_probe(datagram),
            "for {:?}",
            String::from_utf8_lossy(datagram)
        );
    }
}
