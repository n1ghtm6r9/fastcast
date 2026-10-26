use crate::settings::LogLevel;

pub const LEVEL_CHOICES: [(LogLevel, &str); 3] = [
    (LogLevel::Info, "Обычная"),
    (LogLevel::Debug, "Подробная"),
    (LogLevel::Trace, "Полная"),
];
