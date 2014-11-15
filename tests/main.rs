extern crate traq;
use time::strptime;

#[test]
fn test_add_one() {
  assert_eq!(2, traq::add_one(1));
}