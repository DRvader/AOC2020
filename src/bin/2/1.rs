use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("2.txt")?;

  let mut valid_count = 0;
  for line in buf.lines() {
    let mut tokens = line.split(":");
    let mut range_val = tokens.next().unwrap().split_whitespace();
    let mut range = range_val.next().unwrap().split("-");
    let start: u32 = range.next().unwrap().parse()?;
    let end: u32 = range.next().unwrap().parse()?;
    let val = range_val.next().unwrap().chars().next().unwrap();
    let string = tokens.next().unwrap();

    // let mut map = HashMap::<_, u32>::new();
    // for key in string.chars() {
    //   let count = map.entry(key).or_insert(0);
    //   *count += 1;
    // }
    let mut count = 0;
    for key in string.chars() {
      if key == val {
        count += 1;
      }
    }

    // let count = *map.get(&val).unwrap_or(&0);
    if count >= start && count <= end {
      valid_count += 1;
    }
  }
  println!("{}", valid_count);

  Ok(())
}
