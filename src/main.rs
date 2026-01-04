use std::{net::Ipv4Addr, process};

use rusty_config::Config;
use rusty_http::{HttpStatus, Request, Response};
use rusty_router::Router;
use rusty_server::{Server, ServerConfig};
use rusty_utils::init_logger;
use tracing::{debug, error};

fn main() {
    init_logger();

    let config: ServerConfig = ServerConfig {
        port: Config::from_env("PORT").unwrap_or(3000),
        pool_size: Config::from_env("POOL_SIZE").unwrap_or(100),
        host: Config::from_env("HOST").unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
    };

    let mut router: Router = Router::new();

    router.get("/ping", |_request: Request, response: Response| {
        debug!("pong!");
        response.send(HttpStatus::Ok);
    });

    match Server::new(router, config) {
        Ok(server) => {
            server.listen();
        }
        Err(e) => {
            error!("Failed to start server: {e}");
            process::exit(1);
        }
    }
}
