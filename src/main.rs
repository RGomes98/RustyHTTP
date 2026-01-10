use std::net::Ipv4Addr;

use rusty_config::Config;
use rusty_http::{Headers, HttpStatus, Params, Request, Response};
use rusty_router::Router;
use rusty_server::{Server, ServerConfig};

fn main() {
    let config: ServerConfig = ServerConfig {
        port: Config::from_env("PORT").unwrap_or(3000),
        pool_size: Config::from_env("POOL_SIZE").unwrap_or(100),
        host: Config::from_env("HOST").unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
    };

    let mut router: Router = Router::new();

    router.get("/ping", |req: Request, res: Response| {
        let headers: Headers = req.headers;
        println!("Headers: {headers:#?}");
        println!("pong!");
        res.send(HttpStatus::Ok);
    });

    router.get("/store/:store_id/customer/:customer_id", |req: Request, res: Response| {
        let params: Params = req.params;
        println!("Params: {params:#?}");
        res.send(HttpStatus::Ok);
    });

    Server::new(router, config)
        .expect("Failed to initialize server")
        .with_default_logger()
        .listen();
}
