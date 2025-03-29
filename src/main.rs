mod modules;

use modules::config::Env;
use modules::http::HttpMethod;
use modules::router::{Route, Router};
use modules::server::{Config, Server};

fn main() {
    let host: String = Env::get_env_var_or_exit("HOST");
    let port: String = Env::get_env_var_or_exit("PORT");

    Router::new(vec![
        Route {
            method: HttpMethod::GET,
            path: String::from("/"),
            handler: |request, response| println!("Handling request to route 1"),
        },
        Route {
            method: HttpMethod::POST,
            path: String::from("/"),
            handler: |request, response| println!("Handling request to route 2"),
        },
    ]);

    Server::new(Config { host, port });
}
