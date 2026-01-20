use std::{net::Ipv4Addr, thread::sleep, time::Duration};

use forge::prelude::{
    Config, Headers, HttpError, HttpStatus, Listener, ListenerOptions, Params, Request, Response, Router, get, routes,
};

fn main() {
    let mut router: Router = Router::new();

    let config: ListenerOptions = ListenerOptions {
        port: Config::from_env("PORT").unwrap_or(3000),
        host: Config::from_env("HOST").unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1)),
    };

    routes!(router, {
        get "/ping" => ping_handler,
        get "/health" => |_| { Ok(Response::new(HttpStatus::Ok).body("OK")) },
        get "/john_doe" => async |_| { Ok(Response::new(HttpStatus::Ok).body(get_user().await)) },
    });

    get!(router, "/store/:store_id/customer/:customer_id", |request: Request| {
        let params: Params = request.params;
        println!("Params: {params:#?}");
        Ok(Response::new(HttpStatus::Ok))
    });

    Listener::new(router, config)
        .expect("Failed to initialize server")
        .with_default_logger()
        .run();
}

fn ping_handler(request: Request) -> Result<Response, HttpError> {
    let headers: Headers = request.headers;
    println!("Headers: {headers:#?}");
    Ok(Response::new(HttpStatus::Ok).body("pong!"))
}

async fn get_user() -> &'static str {
    sleep(Duration::from_secs(5));
    "John Doe"
}
