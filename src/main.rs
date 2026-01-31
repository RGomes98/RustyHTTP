use std::{net::Ipv4Addr, time::Duration};

use forge::prelude::*;
use serde_json::json;
use tokio::time::sleep;

#[forge::prelude::main]
async fn main() {
    let mut router: Router = Router::new();

    let config: ListenerOptions = ListenerOptions {
        port: Config::from_env("PORT").unwrap_or(3000),
        host: Config::from_env("HOST").unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
    };

    routes!(router, {
        get "/user" => user_handler,
        get "/ping" => ping_handler,
        get "/health" => health_handler,
    });

    get!(router, "/store/:store_id/customer/:customer_id", |req: Request| {
        let params: Params = req.params;
        println!("Params: {params:#?}");
        Response::new(HttpStatus::Ok)
    });

    if let Err(e) = Listener::new(router, config).with_default_logger().run().await {
        eprintln!("Failed to initialize server {e}")
    };
}

async fn user_handler(_: Request<'_>) -> Response<'_> {
    sleep(Duration::from_secs(5)).await;
    let user: serde_json::Value = json!({ "name": "John Doe", "age": 18 });
    Response::new(HttpStatus::Ok).json(user)
}

fn ping_handler(req: Request) -> Response {
    let headers: Headers = req.headers;
    println!("Headers: {headers:#?}");
    Response::new(HttpStatus::Ok).body("pong!")
}

fn health_handler(_: Request) -> Response {
    Response::new(HttpStatus::Ok).body("OK")
}
