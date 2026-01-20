use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("Route already exists: {0}")]
    DuplicateRoute(String),
}
