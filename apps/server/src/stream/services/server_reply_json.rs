use crate::stream::{DISCOVERY_VERSION, SERVER_REPLY_KIND, ServerReply};

pub fn server_reply_json(name: &str, port: u16) -> String {
    serde_json::to_string(&ServerReply {
        kind: SERVER_REPLY_KIND,
        version: DISCOVERY_VERSION,
        name,
        port,
    })
    .expect("a reply made of plain strings and numbers always serializes")
}
