use std::net::Ipv4Addr;

use rusty_config::Config;
use rusty_http::{Headers, HttpError, HttpStatus, Params, Request, Response};
use rusty_router::{Router, get, routes};
use rusty_server::{Server, ServerConfig};

async fn ping_handler(request: Request<'_>) -> Result<Response<'static>, HttpError> {
    let headers: Headers = request.headers;
    println!("Headers: {headers:#?}");
    Ok(Response::new(HttpStatus::Ok).body("pong!"))
}

fn main() {
    let mut router: Router = Router::new();

    let config: ServerConfig = ServerConfig {
        port: Config::from_env("PORT").unwrap_or(3000),
        pool_size: Config::from_env("POOL_SIZE").unwrap_or(100),
        host: Config::from_env("HOST").unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
    };

    routes!(router, {
        get "/ping" => ping_handler,
        get "/health" => async |_| { Ok(Response::new(HttpStatus::NoContent)) }
    });

    get!(router, "/store/:store_id/customer/:customer_id", async |request: Request| {
        let params: Params = request.params;
        println!("Params: {params:#?}");
        Ok(Response::new(HttpStatus::Ok))
    });

    Server::new(router, config)
        .expect("Failed to initialize server")
        .with_default_logger()
        .listen();
}
