pub fn add_one(x: int) -> int {
  x + 1
}

fn add_two(x: int) -> int {
  add_one(add_one(x))
}

#[cfg(test)]
mod unit {
  use super::add_two;

  #[test]
  fn test_add_two() {
      assert_eq!(3i, add_two(1i));
  }
}