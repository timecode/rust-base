mod prelude;
use prelude::*;

fn setup() {
    // setup code, specific to integration tests below, goes here
    println!("crate setup called");
}

#[test]
fn it_works_from_integration_test_alpha() {
    setup_all();
    crate::setup();
    let result = package::examples::add(0, 1);
    assert_eq!(result, 1);
}
