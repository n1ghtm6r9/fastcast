use serde::Serialize;

#[derive(Serialize)]
pub struct ServerReply<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub version: u32,
    pub name: &'a str,
    pub port: u16,
}
