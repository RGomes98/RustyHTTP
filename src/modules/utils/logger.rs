use chrono::Local;

enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

enum LogColor {
    RED,
    YELLOW,
    BLUE,
    GREEN,
}

pub struct Logger;
impl Logger {
    fn get_log_color(log_color: LogColor) -> u8 {
        match log_color {
            LogColor::RED => 31,
            LogColor::YELLOW => 33,
            LogColor::BLUE => 34,
            LogColor::GREEN => 32,
        }
    }

    fn get_log_level(log_level: LogLevel) -> &'static str {
        match log_level {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::ERROR => "ERROR",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
        }
    }

    fn get_timestamp(date_format: &str) -> String {
        Local::now().format(date_format).to_string()
    }

    fn colorize(log_message: &str, log_color: LogColor) -> String {
        let ansi_code: u8 = Self::get_log_color(log_color);
        format!("\x1b[{ansi_code}m{log_message}\x1b[0m")
    }

    fn log(log_message: &str, log_level: LogLevel, color_enum: LogColor) {
        let timestamp: String = Self::get_timestamp("%Y-%m-%dT%H:%M:%S");
        let colored_log_level: String = Self::colorize(Self::get_log_level(log_level), color_enum);
        let log_entry: String = format!("[{timestamp}] [{colored_log_level}] - {log_message}");
        println!("{log_entry}");
    }

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
}
