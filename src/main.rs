use std::net::Ipv4Addr;

use rusty_config::Config;
use rusty_http::{Headers, HttpStatus, Params, Request, Response};
use rusty_router::Router;
use rusty_server::{Server, ServerConfig};

fn main() {
    let mut router: Router = Router::new();

    let config: ServerConfig = ServerConfig {
        port: Config::from_env("PORT").unwrap_or(3000),
        pool_size: Config::from_env("POOL_SIZE").unwrap_or(100),
        host: Config::from_env("HOST").unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
    };

    router.get("/ping", |request: Request| {
        let headers: Headers = request.headers;
        println!("Headers: {headers:#?}");
        println!("pong!");
        Ok(Response::new(HttpStatus::Ok))
    });

    router.get("/store/:store_id/customer/:customer_id", |request: Request| {
        let params: Params = request.params;
        println!("Params: {params:#?}");
        Ok(Response::new(HttpStatus::Ok))
    });

    Server::new(router, config)
        .expect("Failed to initialize server")
        .with_default_logger()
        .listen();
}
