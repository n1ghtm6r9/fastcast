use crate::discovery::is_discovery_probe;

#[test]
fn json_of_another_shape_or_type_is_not_a_probe() {
    let documents: [&[u8]; 11] = [
        br#"{"type":"server","version":1,"name":"KOMPUTER","port":3278}"#,
        br#"{"type":"Discover","version":1}"#,
        br#"{"type":" discover","version":1}"#,
        br#"{"type":["discover"]}"#,
        br#"{"type":null}"#,
        br#"{"version":1}"#,
        br#"{}"#,
        br#"["discover"]"#,
        br#""discover""#,
        b"42",
        b"null",
    ];

    for document in documents {
        assert!(
            !is_discovery_probe(document),
            "for {}",
            String::from_utf8_lossy(document)
        );
    }
}
