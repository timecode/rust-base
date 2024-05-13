use crate::prelude::*;

#[test]
fn it_works_from_integration_test_a1() {
    setup_all();
    crate::setup();
    let result = package::examples::add(3, 4);
    assert_eq!(result, 7);
}
