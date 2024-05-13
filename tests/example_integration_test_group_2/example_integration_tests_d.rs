use crate::prelude::*;

#[test]
fn it_works_from_integration_test_d1() {
    setup_all();
    crate::setup();
    let result = package::examples::add(17, 2);
    assert_eq!(result, 19);
}
