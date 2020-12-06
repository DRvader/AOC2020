use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("6.txt")? + "\n";

  let mut answers = 0;
  let mut map = HashSet::new();
  let mut first = true;
  for line in buf.lines() {
    if line.len() == 0 {
      answers += map.len();
      map.clear();
      first = true;
      continue;
    }

    if first {
      for char in line.chars() {
        map.insert(char);
      }
      first = false;
    } else {
      let mut person_map = HashSet::new();
      for char in line.chars() {
        person_map.insert(char);
      }
      map = map.intersection(&person_map).copied().collect();
    }
  }
  println!("{}", answers);

  Ok(())
}
