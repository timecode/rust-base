//! Crate prelude

// Re-export the crate errors
pub use crate::errors::*;

// Alias Result to be the crate Result
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for new type pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);
