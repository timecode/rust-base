mod prelude;
use prelude::*;

mod example_integration_test_group_1 {
    mod example_integration_tests_a;
    mod example_integration_tests_b;
}

fn setup() {
    // setup code, specific to integration tests below and in this module group, goes here
    println!("group 1 setup called");
}

#[test]
fn it_works_from_integration_test_group_1() {
    setup_all();
    crate::setup();
    let result = package::examples::add(2, 3);
    assert_eq!(result, 5);
}
