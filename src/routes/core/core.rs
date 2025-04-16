use crate::modules::http::{HttpMethod, HttpStatus, Request, Response};
use crate::modules::router::Route;

use std::thread;
use std::time::Duration;

pub fn routes() -> [Route; 3] {
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
        Route {
            path: "/delay",
            handler: delay,
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

fn delay(request: Request, response: Response) {
    println!("Processing with 5 seconds delay...");
    thread::sleep(Duration::from_secs(5));
    println!("Request processing completed.");
    response.send(HttpStatus::Ok);
}
