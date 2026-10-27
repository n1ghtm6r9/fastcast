use serde::Serialize;

#[derive(Serialize)]
pub struct DiagnoseNotice<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub run: u64,
}
