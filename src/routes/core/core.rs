use crate::modules::http::{HttpMethod, Request, Response};
use crate::modules::router::Route;

pub fn routes() -> [Route; 2] {
    [
        Route {
            path: "/version",
            handler: version,
            method: HttpMethod::GET,
        },
        Route {
            path: "/ping",
            handler: ping,
            method: HttpMethod::GET,
        },
    ]
}

fn version(request: Request, response: Option<Response>) {
    println!("Version - [0.1.0]");
}

fn ping(request: Request, response: Option<Response>) {
    println!(
        "Ping received: [{}] - '{}'",
        request.request_line.method, request.request_line.path
    );
}
