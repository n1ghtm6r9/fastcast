use crate::support::{
    connect_viewer, hub_with_nvenc_config, next_config, nvenc_avcc, start_server,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::collections::BTreeSet;

#[tokio::test]
async fn the_config_message_describes_the_stream() {
    let server = start_server(hub_with_nvenc_config()).await;
    let mut viewer = connect_viewer(server.address).await;

    let config = next_config(&mut viewer).await;
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
    assert_eq!(config["codec"], "avc1.640033");
    assert_eq!(config["width"], 2560);
    assert_eq!(config["height"], 1440);
    assert_eq!(config["fps"], 60);
    assert_eq!(description, nvenc_avcc());
}
