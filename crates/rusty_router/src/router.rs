use std::collections::HashMap;

use super::RouterError;
use crate::method_impl;
use rusty_http::{HttpMethod, Request, Response};
use tracing::{debug, trace, warn};

type Path = &'static str;
type RouteMap = HashMap<Path, Route>;
type Routes = HashMap<HttpMethod, RouteMap>; // TODO: Refactor to support dynamic routes (wildcards)

pub struct Route {
    pub path: Path,
    pub method: HttpMethod,
    pub handler: Box<dyn Fn(Request, Response) + Send + Sync>,
}

pub struct Router {
    routes: Routes,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        trace!("Initializing router");
        Self { routes: HashMap::new() }
    }

    pub fn register<F>(&mut self, method: HttpMethod, path: &'static str, handler: F)
    where
        F: Fn(Request, Response) + Send + Sync + 'static,
    {
        self.add_route(Route {
            path,
            method,
            handler: Box::new(handler),
        })
        .expect("Fatal error registering route");
    }

    pub fn get_route(&self, path: &str, method: &HttpMethod) -> Option<&Route> {
        trace!("Looking up route for {method} {path}");

        let route_map: &RouteMap = self.routes.get(method)?;
        let route: Option<&Route> = route_map.get(path);

        if route.is_some() {
            trace!("Route found: {}", Self::fmt_route(method, path));
        } else {
            debug!("No route match found for {}", Self::fmt_route(method, path));
        }

        route
    }

    method_impl!(get, HttpMethod::GET);

    method_impl!(post, HttpMethod::POST);

    method_impl!(put, HttpMethod::PUT);

    method_impl!(delete, HttpMethod::DELETE);

    method_impl!(patch, HttpMethod::PATCH);

    method_impl!(head, HttpMethod::HEAD);

    method_impl!(options, HttpMethod::OPTIONS);

    method_impl!(trace, HttpMethod::TRACE);

    fn add_route(&mut self, route: Route) -> Result<(), RouterError> {
        let route_map: &mut RouteMap = self.routes.entry(route.method).or_default();

        if route_map.contains_key(&route.path) {
            warn!("Route already exists: {}", Self::fmt_route(&route.method, route.path));
            return Err(RouterError::DuplicateRoute(Self::fmt_route(&route.method, route.path)));
        }

        debug!("Registered route: {}", Self::fmt_route(&route.method, route.path,));
        route_map.insert(route.path, route);
        Ok(())
    }

    fn fmt_route(method: &HttpMethod, path: &str) -> String {
        format!("[{method}] - '{path}'")
    }
}
