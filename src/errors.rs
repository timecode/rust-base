//! Main Crate Error

use thiserror::Error;

/// TODO Refactor / re-group into more specifically-named enums as development evolves
#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    /// Wrapped external error
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
