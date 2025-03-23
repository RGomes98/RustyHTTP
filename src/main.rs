mod modules;

use modules::config::Env;
use modules::http::HttpServer;
use modules::utils::Logger;
use std::process;

fn main() {
    let host: String = Env::get_env_var_or_exit("HOST");
    let port: String = Env::get_env_var_or_exit("PORT");

    match HttpServer::new(port, host) {
        Ok(server) => {
            Logger::info(&format!("Server is now listening on {}.", server.address));
            server.handle_connection();
        }
        Err(error) => {
            Logger::error(&format!("Server startup failed. Error: {error}"));
            process::exit(1)
        }
    }
}
