mod modules;
mod routes;

use modules::config::Env;
use modules::router::Router;
use modules::server::{Config, Server};

use routes::core::core_routes;

fn main() {
    let host: String = Env::get_env_var_or_exit("HOST");
    let port: String = Env::get_env_var_or_exit("PORT");

    Router::new(Router::initialize_modules([core_routes()]));
    Server::new(Config { host, port });
}
