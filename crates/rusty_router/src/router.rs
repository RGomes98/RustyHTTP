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

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_http::{HttpStatus, Request, Response};

    fn dummy_handler(_req: Request) -> Result<Response, HttpError> {
        Ok(Response::new(HttpStatus::Ok))
    }

    #[test]
    fn test_basic_static_route_match() {
        let mut router: Router = Router::new();
        router.get("/ping", dummy_handler);

        let result: Option<PathMatch<Handler>> = router.get_route("/ping", &HttpMethod::GET);
        assert!(result.is_some());

        let match_data: PathMatch<Handler> = result.unwrap();
        assert!(match_data.params.is_empty());
    }

    #[test]
    fn test_route_not_found() {
        let mut router: Router = Router::new();
        router.get("/ping", dummy_handler);

        let result: Option<PathMatch<Handler>> = router.get_route("/pong", &HttpMethod::GET);
        assert!(result.is_none());
    }

    #[test]
    fn test_method_mismatch() {
        let mut router: Router = Router::new();
        router.get("/data", dummy_handler);

        let result_get: Option<PathMatch<Handler>> = router.get_route("/data", &HttpMethod::GET);
        assert!(result_get.is_some());

        let result_post: Option<PathMatch<Handler>> = router.get_route("/data", &HttpMethod::POST);
        assert!(result_post.is_none());
    }

    #[test]
    fn test_single_parameter_extraction() {
        let mut router: Router = Router::new();
        router.get("/users/:id", dummy_handler);

        let result: Option<PathMatch<Handler>> = router.get_route("/users/123", &HttpMethod::GET);
        assert!(result.is_some());

        let match_data: PathMatch<Handler> = result.unwrap();
        assert_eq!(match_data.params.len(), 1);
        assert_eq!(match_data.params[0], ("id", "123"));
    }

    #[test]
    fn test_multiple_parameters_extraction() {
        let mut router: Router = Router::new();
        router.get("/store/:store_id/customer/:customer_id", dummy_handler);

        let result: Option<PathMatch<Handler>> = router.get_route("/store/99/customer/500", &HttpMethod::GET);
        assert!(result.is_some());

        let match_data: PathMatch<Handler> = result.unwrap();
        assert_eq!(match_data.params.len(), 2);

        let has_store: bool = match_data.params.contains(&("store_id", "99"));
        let has_customer: bool = match_data.params.contains(&("customer_id", "500"));

        assert!(has_store);
        assert!(has_customer);
    }

    #[test]
    fn test_path_sanitization_and_trailing_slashes() {
        let mut router: Router = Router::new();
        router.get("/api/v1/status", dummy_handler);

        let paths_to_test: Vec<&str> = vec![
            "/api/v1/status",
            "api/v1/status",
            "/api/v1/status/",
            "//api/v1/status//",
        ];

        for path in paths_to_test {
            let result: Option<PathMatch<Handler>> = router.get_route(path, &HttpMethod::GET);
            assert!(result.is_some(), "Failed to match path: {path}");
        }
    }

    #[test]
    fn test_deep_nested_static_routes() {
        let mut router: Router = Router::new();
        router.get("/a/b/c/d", dummy_handler);

        let result: Option<PathMatch<Handler>> = router.get_route("/a/b/c/d", &HttpMethod::GET);
        assert!(result.is_some());

        let partial: Option<PathMatch<Handler>> = router.get_route("/a/b/c", &HttpMethod::GET);
        assert!(partial.is_none());
    }

    #[test]
    fn test_mixed_exact_and_param_segments() {
        let mut router: Router = Router::new();
        router.get("/files/:type/recent", dummy_handler);

        let result: Option<PathMatch<Handler>> = router.get_route("/files/images/recent", &HttpMethod::GET);
        assert!(result.is_some());
        assert_eq!(result.unwrap().params[0], ("type", "images"));

        let result_fail: Option<PathMatch<Handler>> = router.get_route("/files/images/old", &HttpMethod::GET);
        assert!(result_fail.is_none());
    }

    #[test]
    #[should_panic(expected = "Fatal error registering route")]
    fn test_duplicate_route_panics() {
        let mut router: Router = Router::new();
        router.get("/duplicate", dummy_handler);
        router.get("/duplicate", dummy_handler);
    }

    #[test]
    fn test_overlapping_routes_precedence() {
        let mut router: Router = Router::new();

        router.get("/users/all", dummy_handler);
        router.get("/users/:id", dummy_handler);

        let exact_match: Option<PathMatch<Handler>> = router.get_route("/users/all", &HttpMethod::GET);
        assert!(exact_match.is_some());
        assert!(exact_match.unwrap().params.is_empty());

        let param_match: Option<PathMatch<Handler>> = router.get_route("/users/123", &HttpMethod::GET);
        assert!(param_match.is_some());
        assert_eq!(param_match.unwrap().params[0], ("id", "123"));
    }
}
