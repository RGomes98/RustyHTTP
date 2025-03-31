use crate::modules::{
    http::{HttpMethod, Request, Response},
    router::Route,
};

pub fn core_routes() -> [Route; 2] {
    [
        Route {
            handler: version,
            method: HttpMethod::GET,
            path: String::from("/version"),
        },
        Route {
            handler: ping,
            method: HttpMethod::GET,
            path: String::from("/ping"),
        },
    ]
}

fn version(request: Request, response: Option<Response>) {
    println!("Version - [0.1.0]");
}

fn ping(request: Request, response: Option<Response>) {
    println!("Ping received: [{}] - '{}'", request.method, request.path);
}
