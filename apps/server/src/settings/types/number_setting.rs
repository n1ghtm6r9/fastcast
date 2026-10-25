#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberSetting {
    pub variable: &'static str,
    pub default: u32,
    pub min: u32,
    pub max: u32,
}
