mod modules;
mod routes;

use modules::config::Env;
use modules::entry::{App, Config};
use modules::router::Router;

use routes::core;

fn main() {
    let host: String = Env::get_env_var_or_exit("HOST");
    let port: String = Env::get_env_var_or_exit("PORT");
    Router::new(Router::initialize_modules([core::routes()]));
    App::new(Config { host, port });
}
