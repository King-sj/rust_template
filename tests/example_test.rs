/// here we are testing the add function from the rust_template crate
use rust_template::add::*;

#[test]
fn it_adds_two() {
  add();
  assert_eq!(4, 2 + 2);
}
