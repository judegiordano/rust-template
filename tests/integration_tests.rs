use std::{
    thread,
    time::{self, Duration},
};

fn add(num_one: u8, num_two: u8) -> u8 {
    num_one + num_two
}

#[test]
fn sum_integration() {
    assert_eq!(2, add(1, 1));
    assert_ne!(2, add(2, 2));
}

#[test]
#[ignore]
fn expensive_test_integration() {
    // expensive operation
    let five_seconds: Duration = time::Duration::from_secs(5);
    thread::sleep(five_seconds);
}
