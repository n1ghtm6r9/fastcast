use crate::discovery::server_reply_json;

#[test]
fn the_reply_is_the_server_type_the_protocol_version_the_name_and_the_web_port() {
    assert_eq!(
        server_reply_json("KOMPUTER", 3278),
        r#"{"type":"server","version":1,"name":"KOMPUTER","port":3278}"#
    );
}
