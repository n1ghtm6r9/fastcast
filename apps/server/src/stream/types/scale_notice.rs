use serde::Serialize;

#[derive(Serialize)]
pub struct ScaleNotice<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub percent: u32,
    pub min: u32,
    pub max: u32,
}
