# Base Template for a New Rust Package

Inspired by [rust-base](https://github.com/jeremychone-channel/rust-base) this repo is intended as a starting point for developing a new Rust package. Opinionated editor and formatting rules expect [VSCode](https://code.visualstudio.com/) as the editor and [prettier](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode) as the formatter (and, of course, [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) providing Rust language support).

Everything below "works on my machine!"; currently macOS | Apple Silicon, however mileage shouldn't vary much on other setups.

## Usage

### 1. Clone

In a new terminal window/tab, `cd` to the directory that will be the parent of the new package. Choose a name for the new package ...

```sh
export NEW_PACKAGE_NAME="set-new-package-name-here"
```

Then copy/paste and execute the following ...

```sh
git clone https://github.com/timecode/rust-base.git ${NEW_PACKAGE_NAME} && \
cd ${NEW_PACKAGE_NAME} && \
sed -E -i '' "s/rust-base/${NEW_PACKAGE_NAME}/g" Cargo.toml && \
sed -E -i '' "s/rust_base/${NEW_PACKAGE_NAME//-/_}/g" \
    src/bin/main.rs src/examples.rs tests/prelude/mod.rs && \
rm -rf .git && git init
```

#### Verify

Open the new package directory in a code editor (VSCode).
Run the included default test(s), either directly from VSCode's built in runner or via a new terminal window (Ctrl-Shift-` in VSCode), with:

```sh
cargo test
```

All tests should run and pass OK.

From this point, feel free to modify, replace, or remove [`src/bin/main.rs`](./src/bin/main.rs), [`src/examples.rs`](./src/examples.rs), [`tests/`](./tests/), etc, and of course the contents of this [README](./README.md) file.

Also, see note in [`.gitignore#L43`](./.gitignore#L43) regarding `Cargo.lock` inclusion / exclusion going forward.

### 2. Develop

This is currently how I roll (i.e. develop code):

1.  In a separate terminal window/tab (`cd` to the package directory, if required), invoke `$ bacon`
    -   if required, install with `$ brew install bacon`
    -   see the `bacon` [website](https://dystroy.org/bacon/) for full details
2.  Start coding (see 'Dev Tips' below) ... mostly, activities happen in the editor (with `bacon` running off to the side somewhere). Although, every now and then the following may get used ...
    -   as required:
        -   `cargo build`
        -   `cargo run [--bin <a-binary-name>]`
    -   more rarely:
        -   `cargo doc [--open]`
        -   `cargo fmt` to lint (although vscode should be doing this all the time anyway)
        -   `cargo check [--bin <a-binary-name>]` (prior to a build and run)
        -   `cargo upgrade -i allow` upgrade `Cargo.toml` to require latest versions of dependencies
        -   `cargo update` to update dependencies in `Cargo.lock`

## Dev Tips

### Mute some warnings (as required)

Use overrides to temporarily keep clippy quiet. For example:

```rust
#![allow(clippy::nursery)]
#![allow(clippy::pedantic)]
#![allow(dead_code)]
#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
```

Paste that lot at the top of each file, then comment / un-comment as required.
Comment out or remove completely once development is complete.

### Write tests before writing code

1.  For each function or library API entrypoint, decide what it's required to 'do'; not in any detail, just "Given 'some input(s)', I want 'a particular output'". That will be the requirement 'contract' to be fulfilled. For example:

    > -   Given some integer, I want that value multiplied by 4.
    > -   If the value can't be multiplied by 4, I want some kind of error instead

2.  Use that information to write a function signature. The naming and types of any function parameters isn't important; they can be renamed as development progresses. For example:

    ```rust
    fn multiply_by_four(x: i32) -> Result<i32> {}
    ```

    **Note**: the function in this example is returning the `Result` defined in this repo i.e. `Result<T>` (which is an alias of `core::result::Result<T, Error>`)

3.  To that signature, add just enough code for the Rust compiler to allow the function to compile; i.e. simply adding something like `Ok(42)` here will satisfy the return expectation. For example:

    ```rust
    fn multiply_by_four(x: i32) -> Result<i32> {
        Ok(42)
    }
    ```

4.  In the test module section at the foot of the file (use `./src/examples.rs` as a template), add a test to show what 'good' looks like. For example:

    ```rust
    #[test]
    fn check_multiply_by_four() -> Result<()> {
        let result = multiply_by_four(2)?;
        Ok(assert_eq!(result, 8))
    }
    ```

    **Note**: as the function returns a `Result`, make use of the question mark operator and return `Result<()>` from the test. That way, any error situations are handled cleanly™.

5.  Add code to the function in order to make the test pass.

    ```rust
    fn multiply_by_four(x: i32) -> Result<()> {
        let val = x * 4;
        Ok(val)
    }
    ```

6.  Add a test to show what 'bad' looks like. For example:

    ```rust
    #[test]
    fn check_multiply_by_four_errors_given_zero() {
        let result = multiply_by_four(0);
        assert!(result.is_err(), "should not like zero!")
    }
    ```

    **Note**: In this (convoluted) example, an error is to be returned under certain conditions.
    It's not what's strictly mentioned in the requirements, but returning an error given a `0` will suffice for now and can be clarified (and possibly even removed) later.

7.  Add code to the function in order to make the test pass.

    ```rust
    fn multiply_by_four(x: i32) -> Result<()> {
        if x == 0 {
            return Err("I don't do zero values!")
        }
        let val = x * 4;
        Ok(val)
    }
    ```

8.  Clarify requirements and modify expectations in the tests for what both 'good' and 'bad' look like, as and when required; ensuring that all tests continue to pass after any changes. That goes for any refactoring too.

**Note**: Various 'integration test' options, corresponding file structure, and grouping possibilities, are provided in the example [`tests/`](./tests/) directory.
