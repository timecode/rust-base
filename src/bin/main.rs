//! Example binary
//! Feel free to use as a reference and/or delete

use rust_base::prelude::*;

/// `main` is setup here to provide an example of using this package's custom `Result` and `Error`
///
/// # Errors
///
/// Will return `Err` if ...
pub fn main() -> Result<()> {
    let message = format!("Hello, {}", "World!");
    println!("{message}");

    rust_base::examples::list_dir()?;

    Ok(println!("... complete"))
}

/* ********************************************************************************************** */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_main() -> Result<()> {
        main()
    }
}
