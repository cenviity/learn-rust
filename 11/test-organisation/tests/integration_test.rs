use test_organisation;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organisation::add_two(2));
}
