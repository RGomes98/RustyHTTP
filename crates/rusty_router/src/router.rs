use std::collections::HashMap;

use super::RouterError;
use rusty_http::{HttpMethod, Request, Response};
use tracing::{debug, error, trace, warn};

type Path = &'static str;
type RouteMap = HashMap<Path, Route>;
type Routes = HashMap<HttpMethod, RouteMap>; // TODO: Refactor to support dynamic routes (wildcards)

pub struct Route {
    pub path: Path,
    pub method: HttpMethod,
    pub handler: Box<dyn Fn(Request, Response)>,
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

    pub fn try_add_route(&mut self, route: Route) -> Result<(), RouterError> {
        let route_map: &mut RouteMap = self.routes.entry(route.method).or_default();

        if route_map.contains_key(&route.path) {
            warn!("Route already exists: {}", Self::fmt_route(&route.method, route.path));
            return Err(RouterError::DuplicateRoute(Self::fmt_route(&route.method, route.path)));
        }

        debug!("Registered route: {}", Self::fmt_route(&route.method, route.path,));
        route_map.insert(route.path, route);
        Ok(())
    }

    pub fn add_route(&mut self, route: Route) {
        self.try_add_route(route).unwrap_or_else(|e: RouterError| {
            error!("Fatal error: Failed to register route: {e}");
            panic!("Fatal error registering route: {e}")
        });
    }

    fn fmt_route(method: &HttpMethod, path: &str) -> String {
        format!("[{method}] - '{path}'")
    }
}
