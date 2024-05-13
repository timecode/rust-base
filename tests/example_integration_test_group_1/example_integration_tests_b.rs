use crate::prelude::*;

#[test]
fn it_works_from_integration_test_b1() {
    setup_all();
    crate::setup();
    let result = package::examples::add(7, 4);
    assert_eq!(result, 11);
}

#[test]
fn it_works_from_integration_test_b2() {
    setup_all();
    crate::setup();
    let result = package::examples::add(11, 2);
    assert_eq!(result, 13);
}
