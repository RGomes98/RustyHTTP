use std::collections::HashMap;

pub struct Logger;
#[derive(Hash, Eq, PartialEq, Debug)]
enum Color {
    Red,
    Yellow,
    Blue,
    Green,
}

impl Logger {
    fn log(level: &str, message: &str) {
        let log_entry = format!("[{}] {}\n", level, message);
        println!("{}", log_entry.trim());
    }

    fn debug(message: &str) -> () {
        Self::log("DEBUG", message);
    }

    fn info(message: &str) -> () {
        Self::log("INFO", message);
    }

    fn warn(message: &str) -> () {
        Self::log("WARN", message);
    }

    fn error(message: &str) -> () {
        Self::log("ERROR", &Self::colorize(message, &Color::Red));
    }

    fn colorize(message: &str, color: &Color) -> String {
        let mut map: HashMap<Color, i32> = HashMap::from([
            (Color::Red, 31),
            (Color::Yellow, 33),
            (Color::Blue, 34),
            (Color::Green, 32),
        ]);

        return format!(
            "\x1b[{}m{}\x1b[0m",
            map.get(color).expect("msg").to_string(),
            message
        );
    }
}
