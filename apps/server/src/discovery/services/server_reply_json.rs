use crate::discovery::{PROTOCOL_VERSION, SERVER_REPLY_KIND, ServerReply};

pub fn server_reply_json(name: &str, web_port: u16) -> String {
    serde_json::to_string(&ServerReply {
        kind: SERVER_REPLY_KIND,
        version: PROTOCOL_VERSION,
        name,
        port: web_port,
    })
    .expect("a reply made of plain strings and numbers always serializes")
}
