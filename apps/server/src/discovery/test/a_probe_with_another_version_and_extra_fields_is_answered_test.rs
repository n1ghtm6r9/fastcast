use super::{bind_prober, next_datagram, start_responder};

#[tokio::test]
async fn a_probe_with_another_version_and_extra_fields_is_answered() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;
    let probes = [
        r#"{"type":"discover","version":2}"#,
        r#"{"type":"discover","version":"next"}"#,
        r#"{"type":"discover"}"#,
        r#"{"version":1,"client":{"app":"tv","tags":[1,2,3]},"type":"discover"}"#,
    ];

    for probe in probes {
        prober
            .send_to(probe.as_bytes(), responder.address)
            .await
            .expect("the probe is sent");

        let (reply, _) = next_datagram(&prober).await;

        assert_eq!(
            reply, r#"{"type":"server","version":1,"name":"KOMPUTER","port":3278}"#,
            "for {probe}"
        );
    }
}
