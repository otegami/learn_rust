use rand::Rng;
use std::collections::HashMap;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);

  let secret_number = rand::thread_rng().gen_range(1..101);
  println!("{}", secret_number)
}
