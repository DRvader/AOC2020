use std::{error::Error, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("6.txt")? + "\n";

  let mut answers = 0;
  let mut map = HashSet::new();
  for line in buf.lines() {
    if line.len() == 0 {
      map.clear();
      continue;
    }

    for char in line.chars() {
      if !map.contains(&char) {
        answers += 1;
        map.insert(char);
      }
    }
  }
  println!("{}", answers);

  Ok(())
}
