use chrono::Local;
use std::{fmt, str};

#[derive(Debug)]
enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[derive(Debug)]
enum LogColor {
    RED,
    YELLOW,
    BLUE,
    GREEN,
}

enum LogLevelError {
    Invalid,
}

impl fmt::Display for LogLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid log level.")
    }
}

impl str::FromStr for LogLevel {
    type Err = LogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(LogLevel::DEBUG),
            "INFO" => Ok(LogLevel::INFO),
            "WARN" => Ok(LogLevel::WARN),
            "ERROR" => Ok(LogLevel::ERROR),
            _ => Err(LogLevelError::Invalid),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let m: &str = match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
        };

        write!(f, "{m}")
    }
}

impl From<LogColor> for u8 {
    fn from(v: LogColor) -> Self {
        match v {
            LogColor::RED => 31,
            LogColor::YELLOW => 33,
            LogColor::BLUE => 34,
            LogColor::GREEN => 32,
        }
    }
}

impl fmt::Display for LogColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m: &str = match self {
            LogColor::RED => "RED",
            LogColor::YELLOW => "YELLOW",
            LogColor::BLUE => "BLUE",
            LogColor::GREEN => "GREEN",
        };

        write!(f, "{m}")
    }
}

pub struct Logger;

impl Logger {
    #[cfg(debug_assertions)]
    pub fn debug(log_message: &str) {
        Self::log(log_message, LogLevel::DEBUG, LogColor::GREEN);
    }

    #[cfg(not(debug_assertions))]
    pub fn debug(_: &str) {}

    pub fn info(log_message: &str) {
        Self::log(log_message, LogLevel::INFO, LogColor::BLUE);
    }

    pub fn warn(log_message: &str) {
        Self::log(log_message, LogLevel::WARN, LogColor::YELLOW);
    }

    pub fn error(log_message: &str) {
        Self::log(log_message, LogLevel::ERROR, LogColor::RED);
    }

    fn log(log_message: &str, level_enum: LogLevel, color_enum: LogColor) {
        let timestamp: String = Self::get_timestamp("%Y-%m-%dT%H:%M:%S");
        let log_level: String = Self::colorize(&level_enum.to_string(), color_enum);
        let log_entry: String = format!("[{timestamp}] [{log_level}] - {log_message}");
        println!("{log_entry}");
    }

    fn colorize(log_message: &str, log_color: LogColor) -> String {
        let color_code: u8 = log_color.into();
        format!("\x1b[{color_code}m{log_message}\x1b[0m")
    }

    fn get_timestamp(date_format: &str) -> String {
        Local::now().format(date_format).to_string()
    }
}
