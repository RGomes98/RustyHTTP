mod modules;
mod routes;

use modules::config::Env;
use modules::entry::{App, Config};
use modules::router::Router;

use routes::core;

fn main() {
    let config: Config = Config {
        host: Env::get_parsed("HOST"),
        port: Env::get_parsed("PORT"),
        pool_size: Env::get_parsed("POOL_SIZE"),
    };

    Router::new(Router::initialize_modules([core::routes()]));
    App::new(config);
}
