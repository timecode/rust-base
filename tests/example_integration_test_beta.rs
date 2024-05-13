mod prelude;
use prelude::*;

fn setup() {
    // setup code, specific to integration tests below, goes here
    println!("crate setup called");
}

#[test]
fn it_works_from_integration_test_beta() {
    setup_all();
    crate::setup();
    let result = package::examples::add(1, 2);
    assert_eq!(result, 3);
}
