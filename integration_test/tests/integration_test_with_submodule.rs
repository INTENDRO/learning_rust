use integration_test;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, integration_test::add_two(2));
}