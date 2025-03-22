mod modules;

use modules::env::Env;
use modules::logger::Logger;
use std::net::TcpListener;

fn main() {
    let host: String = Env::get_env_var_or_default("HOST", "localhost");
    let port: String = Env::get_env_var_or_default("PORT", "3000");

    let listener: TcpListener = TcpListener::bind(format!("{host}:{port}")).unwrap();
    Logger::info(&format!("Server is now listening on {host}:{port}."));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
