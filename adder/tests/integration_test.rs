use adder;

mod common;

#[test]
fn it_adds_two() {
    common::this_is_a_helper_fn();
    assert_eq!(adder::add_two(5), 7);
}
