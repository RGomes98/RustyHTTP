use std::collections::HashMap;

use super::RouterError;
use crate::method_impl;
use rusty_http::{HttpError, HttpMethod, Request, Response};
use rusty_utils::{PathMatch, PathTree, Segment};
use tracing::{debug, trace, warn};

type Path = &'static str;
type Routes = HashMap<HttpMethod, PathTree<Handler>>; // TODO: Add support to dynamic routes (wildcards)
pub type Handler = Box<dyn Fn(Request) -> Result<Response, HttpError> + Send + Sync>;

const ROUTER_RULES: (char, char) = ('/', ':');

pub struct Route {
    pub path: Path,
    pub handler: Handler,
    pub method: HttpMethod,
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
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.add_route(Route {
            path,
            method,
            handler: Box::new(handler),
        })
        .expect("Fatal error registering route");
    }

    pub fn get_route<'a, 'b>(&'a self, path: &'b str, method: &HttpMethod) -> Option<PathMatch<'a, 'b, Handler>> {
        trace!("Looking up route for {method} {path}");

        let path_tree: &PathTree<Handler> = self.routes.get(method)?;
        let route: Option<PathMatch<Handler>> = path_tree.find(Self::sanitize_path(path));

        if route.is_some() {
            debug!("Route found: {}", Self::fmt_route(method, path));
        } else {
            debug!("No route match found for {}", Self::fmt_route(method, path));
        }

        route
    }

    fn add_route(&mut self, route: Route) -> Result<(), RouterError> {
        let path_tree: &mut PathTree<Handler> = self.routes.entry(route.method).or_default();

        if path_tree
            .insert(Self::parse_to_segment(route.path), route.handler)
            .is_some()
        {
            warn!("Route already exists: {}", Self::fmt_route(&route.method, route.path));
            return Err(RouterError::DuplicateRoute(Self::fmt_route(&route.method, route.path)));
        };

        debug!("Registered route: {}", Self::fmt_route(&route.method, route.path,));
        Ok(())
    }

    fn parse_to_segment<'a>(path: &'a str) -> impl Iterator<Item = Segment<'a>> {
        Self::sanitize_path(path).map(|path: &str| {
            if path.starts_with(ROUTER_RULES.1) {
                Segment::Param(&path[1..])
            } else {
                Segment::Exact(path)
            }
        })
    }

    fn sanitize_path(path: &str) -> impl Iterator<Item = &str> {
        path.trim_matches(ROUTER_RULES.0)
            .split(ROUTER_RULES.0)
            .filter(|s: &&str| !s.is_empty())
    }

    fn fmt_route(method: &HttpMethod, path: &str) -> String {
        format!("[{method}] - \"{path}\"")
    }

    method_impl!(get, HttpMethod::GET);

    method_impl!(post, HttpMethod::POST);

    method_impl!(put, HttpMethod::PUT);

    method_impl!(delete, HttpMethod::DELETE);

    method_impl!(patch, HttpMethod::PATCH);

    method_impl!(head, HttpMethod::HEAD);

    method_impl!(options, HttpMethod::OPTIONS);

    method_impl!(trace, HttpMethod::TRACE);
}
