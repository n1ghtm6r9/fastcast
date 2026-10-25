use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ViewerQuery {
    pub width: Option<String>,
    pub height: Option<String>,
    pub fps: Option<String>,
    pub codecs: Option<String>,
}
