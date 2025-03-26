mod modules;

use modules::config::Env;
use modules::http::{Config, HttpMethod, HttpServer};
use modules::router::{Route, Router};
use modules::utils::Logger;
use std::process;

fn main() {
    let host: String = Env::get_env_var_or_exit("HOST");
    let port: String = Env::get_env_var_or_exit("PORT");

    let mut router: Router = Router::new();

    //TODO routes mod/server mod
    router.register(Route {
        method: HttpMethod::GET,
        path: String::from("/"),
        handler: |_, _| println!("Handling request"),
    });

    match HttpServer::new(router, Config { port, host }) {
        Ok(server) => {
            Logger::info(&format!(
                "Server is now listening on {}.",
                server.get_address()
            ));

            server.handle_connection();
        }
        Err(error) => {
            Logger::error(&format!("Server startup failed. Error: {error}"));
            process::exit(1)
        }
    }
}
