mod modules;
mod routes;

use modules::config::Env;
use modules::entry::{App, Config};
use modules::router::Router;
use std::net::Ipv4Addr;

use routes::core;

fn main() {
    let host: Ipv4Addr = Env::get_parsed("HOST");
    let port: u16 = Env::get_parsed("PORT");
    Router::new(Router::initialize_modules([core::routes()]));
    App::new(Config { host, port });
}
