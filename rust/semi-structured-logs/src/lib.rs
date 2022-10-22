// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level.as_str(), message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn debug(message: &str) -> String { log(LogLevel::Debug, message) }
pub fn warn(message: &str) -> String { log(LogLevel::Warning, message) }
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
