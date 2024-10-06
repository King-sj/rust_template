/// this is a demo function to show how to create a module in rust
pub fn add() {
  println!("Hello from add function in module add.rs");
}
/// this shows how to write an inner module test
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    add();
    assert_eq!(2 + 2, 4);
  }
}
