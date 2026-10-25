use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigMessage<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub codec: &'a str,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub description: &'a str,
}
