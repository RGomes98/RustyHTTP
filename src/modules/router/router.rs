use crate::modules::{http::HttpMethod, utils::Logger};
use std::{collections::HashMap, io::Error, process};

//TODO Request module
pub struct Request {
    pub message: String,
}

//TODO Response module
pub struct Response {
    pub message: String,
}

pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handler: fn(Request, Response),
}

pub struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    pub fn new() -> Router {
        let routes: HashMap<String, Route> = HashMap::new();
        Router { routes }
    }

    pub fn register(&mut self, new_route: Route) {
        if let Some(Route { method, path, .. }) = self.get_route(&new_route.path, &new_route.method)
        {
            Logger::error(&format!(
                "Error: Route with method '{method}' and path '{path}' is already registered.",
            ));

            process::exit(1);
        }

        self.routes.insert(new_route.path.to_string(), new_route);
    }

    fn get_route<'a>(&self, path: &'a String, method: &'a HttpMethod) -> Option<&Route> {
        match self.routes.get(path) {
            Some(route) if route.method.eq(method) => Some(route),
            _ => None,
        }
    }

    //TODO req/res from http server
    pub fn invoke_route(&self, path: String, method: HttpMethod) -> Result<(), Error> {
        if let Some(route) = self.get_route(&path, &method) {
            //test
            let request: Request = Request {
                message: format!("Request to route [{method}] - {path}."),
            };
            //test
            let response: Response = Response {
                message: format!("Response from route [{method}] - {path}."),
            };

            (route.handler)(request, response);
            Ok(())
        } else {
            Logger::error(&format!(
                "Error: Route with method '{method}' and path '{path}' not found."
            ));

            Err(Error::new(
                //add router errors
                std::io::ErrorKind::NotFound,
                "Error: Route with method '{method}' and path '{path}' not found.",
            ))
        }
    }
}
