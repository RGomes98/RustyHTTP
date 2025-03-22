mod modules;

use modules::config::Env;
use modules::utils::Logger;
use std::net::TcpListener;

fn main() {
    let host: String = Env::get_env_var_or_default("HOST", "127.0.0.1");
    let port: String = Env::get_env_var_or_default("PORT", "8080");

    let listener: TcpListener = TcpListener::bind(format!("{host}:{port}")).unwrap();
    Logger::info(&format!("Server is now listening on {host}:{port}."));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                Logger::info("Connection established!");
            }
            Err(e) => {
                Logger::error(&format!("Failed to accept connection: {e}"));
            }
        }
    }
}
