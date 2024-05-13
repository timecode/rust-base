//! Tests prelude
//! Common code, accessible by all tests, goes here

// Alias the actual package name to "package"
#[allow(unused_imports)]
pub use rust_base as package;

#[allow(dead_code)]
pub fn setup_all() {
    // setup code, specific to the library's tests, would go here
    println!("common setup called");
}
