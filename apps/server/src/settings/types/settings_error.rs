use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsError {
    pub variable: &'static str,
    pub expected: String,
    pub got: String,
}

impl fmt::Display for SettingsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} must be {}, got \"{}\"",
            self.variable, self.expected, self.got
        )
    }
}

impl Error for SettingsError {}
