use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, start_endpoint, x264_avcc,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::collections::BTreeSet;
use wire::SESSION_TIMEOUT;

#[test]
fn the_config_message_describes_the_stream() {
    let endpoint = start_endpoint(hub_with_config(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    let config = next_config(&mut viewer);
    let keys: BTreeSet<&str> = config
        .as_object()
        .expect("the config is an object")
        .keys()
        .map(String::as_str)
        .collect();
    let description = STANDARD
        .decode(
            config["description"]
                .as_str()
                .expect("the description is text"),
        )
        .expect("the description is base64");

    assert_eq!(
        keys,
        BTreeSet::from(["codec", "description", "fps", "height", "type", "width"])
    );
    assert_eq!(config["codec"], "avc1.42c02a");
    assert_eq!(config["width"], 1920);
    assert_eq!(config["height"], 1080);
    assert_eq!(config["fps"], 60);
    assert_eq!(description, x264_avcc());
}
