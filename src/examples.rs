//! Example code
//! Feel free to use as a reference and/or delete

// #![allow(clippy::nursery)]
// #![allow(clippy::pedantic)]
#![allow(dead_code)]
// #![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]

use crate::prelude::*;

/// Lists the current directory
///
/// # Errors
///
/// Will return `Err` if ...
///
pub fn list_dir() -> Result<()> {
    for entry in std::fs::read_dir("./")?.filter_map(std::result::Result::ok) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}",);
    }

    // for sanity check, force an error to be returned using either of the following
    // let error_text = "example from list_dir";
    // Err(Error::Generic(String::from(error_text)))
    // Err(Error::Static(error_text))

    Ok(())
}

/// Adds two given numbers
///
/// # Examples
///
/// ```
/// let answer = rust_base::examples::add(3, 4);
///
/// assert_eq!(answer, 7);
/// ```
///
#[must_use]
pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

const fn add_two(left: usize) -> usize {
    left + 2
}

const fn multiply_by_four(x: i32) -> Result<i32> {
    if x == 0 {
        return Err(Error::Static("I don't do zero values!"));
    }
    let val = x * 4;
    Ok(val)
}

/* ********************************************************************************************** */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_multiply_by_four() -> Result<()> {
        let result = multiply_by_four(2)?;
        Ok(assert_eq!(result, 8))
    }

    #[test]
    fn check_multiply_by_four_errors_given_zero() {
        let result = multiply_by_four(0);
        assert!(result.is_err(), "should not like zero!");
    }

    #[test]
    fn check_list_dir() -> Result<()> {
        list_dir()?;
        Ok(())
    }
}
