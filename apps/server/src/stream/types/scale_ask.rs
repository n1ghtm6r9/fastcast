use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScaleAsk {
    #[serde(rename = "type")]
    pub kind: String,
    pub step: i32,
}
