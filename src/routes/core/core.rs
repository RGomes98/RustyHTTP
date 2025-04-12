use crate::modules::http::{HttpMethod, HttpStatus, Request, Response};
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

fn version(request: Request, response: Response) {
    println!("Version - [0.1.0]");
    response.send(HttpStatus::Ok);
}

fn ping(request: Request, response: Response) {
    println!("Pong!");
    response.send(HttpStatus::Ok);
}
