// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", format!("{:?}", level).to_uppercase(), message)
}

pub fn info(message: &str) -> String { log(LogLevel::Info, message) }
pub fn warn(message: &str) -> String { log(LogLevel::Warning, message) }
pub fn error(message: &str) -> String { log(LogLevel::Error, message) }
