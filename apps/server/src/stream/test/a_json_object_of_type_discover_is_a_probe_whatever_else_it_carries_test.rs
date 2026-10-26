use crate::stream::is_discovery_probe;

#[test]
fn a_json_object_of_type_discover_is_a_probe_whatever_else_it_carries() {
    let probes: [&[u8]; 7] = [
        br#"{"type":"discover","version":1}"#,
        br#"{"type":"discover","version":99}"#,
        br#"{"type":"discover","version":"next"}"#,
        br#"{"type":"discover"}"#,
        br#"{"version":1,"client":{"app":"tv","tags":[1,2,3]},"type":"discover"}"#,
        b" \r\n{ \"type\" : \"discover\" }\r\n",
        br#"{"type":"discover"}"#,
    ];

    for probe in probes {
        assert!(
            is_discovery_probe(probe),
            "for {}",
            String::from_utf8_lossy(probe)
        );
    }
}
