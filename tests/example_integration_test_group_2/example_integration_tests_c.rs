use crate::prelude::*;

#[test]
fn it_works_from_integration_test_c1() {
    setup_all();
    crate::setup();
    let result = package::examples::add(13, 4);
    assert_eq!(result, 17);
}
