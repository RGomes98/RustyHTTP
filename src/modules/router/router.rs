use crate::modules::{http::HttpMethod, http::Request, http::Response, utils::Logger};

use std::fmt;
use std::sync::OnceLock;
use std::{collections::HashMap, process};

static ROUTE_MAP: OnceLock<HashMap<String, Route>> = OnceLock::new();

#[derive(Debug)]
pub struct Route {
    pub path: &'static str,
    pub method: HttpMethod,
    pub handler: fn(Request, Option<Response>),
}

pub enum RouterError {
    RouteNotFound,
    RouterNotInitialized,
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Router Error: {}",
            match self {
                RouterError::RouteNotFound => "Route not found.",
                RouterError::RouterNotInitialized => "Router was not initialized correctly.",
            }
        )
    }
}

pub struct Router;

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        let route_map: HashMap<String, Route> = Self::register_routes(routes);
        let route_count: usize = route_map.len();

        match ROUTE_MAP.set(route_map) {
            Ok(_) => {
                Logger::info(&format!("Initializing {route_count} routes."));
                Logger::info("All routes were initialized successfully!");
            }
            Err(_) => {
                Logger::error("Failed to initialize routes. 'OnceLock' already initialized.");
                process::exit(1);
            }
        }

        Self
    }

    pub fn initialize_modules<const R: usize, const M: usize>(
        modules: [[Route; R]; M],
    ) -> Vec<Route> {
        modules.into_iter().flatten().collect::<Vec<Route>>()
    }

    pub fn get_route_by_identifier(identifier: String) -> Result<&'static Route, RouterError> {
        let route_map: &HashMap<String, Route> = Self::get_route_map()?;

        match route_map.get(&identifier) {
            Some(route) => Ok(route),
            None => Err(RouterError::RouteNotFound),
        }
    }

    pub fn get_route_identifier(path: &str, method: &HttpMethod) -> String {
        format!("[{method}] - '{path}'")
    }

    fn register_routes(routes: Vec<Route>) -> HashMap<String, Route> {
        let mut route_map: HashMap<String, Route> = HashMap::new();

        routes.into_iter().for_each(|route| {
            let idenfitier: String = Self::get_route_identifier(&route.path, &route.method);

            match route_map.get(&idenfitier) {
                Some(_) => {
                    Logger::error(&format!("Route {idenfitier} already exists."));
                    process::exit(1);
                }
                None => route_map.insert(idenfitier, route),
            };
        });

        route_map
    }

    fn get_route_map() -> Result<&'static HashMap<String, Route>, RouterError> {
        match ROUTE_MAP.get() {
            Some(routes) => Ok(routes),
            None => Err(RouterError::RouterNotInitialized),
        }
    }
}
