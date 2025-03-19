use std::net::TcpListener;
use std::env;

fn get_from_env(key:&str, fallback:&str) -> String {
    return env::var(key).unwrap_or(fallback.to_string());
}

fn main() {
    let host: String = get_from_env("HOST", "localhost");
    let port: String = get_from_env("PORT", "3000");
    let listener: TcpListener = TcpListener::bind(format!("{host}:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}