#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSetting {
    pub variable: &'static str,
    pub default: &'static str,
}
