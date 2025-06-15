mod common;

#[test]
fn a_test() {
    common::setup();
    println!("A test");
}

#[test]
fn another_test() {
    common::setup();
    println!("Another test");
}
