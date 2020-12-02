use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("2.txt")?;

  let mut valid_count = 0;
  for line in buf.lines() {
    let mut tokens = line.split(":");
    let mut range_val = tokens.next().unwrap().split_whitespace();
    let mut range = range_val.next().unwrap().split("-");
    let start: usize = range.next().unwrap().parse()?;
    let end: usize = range.next().unwrap().parse()?;
    let val = range_val.next().unwrap().chars().next().unwrap();
    let string = tokens.next().unwrap();

    let pos1 = string.chars().nth(start);
    let pos2 = string.chars().nth(end);

    let valid1 = if let Some(char) = pos1 {
      char == val
    } else {
      false
    };

    let valid2 = if let Some(char) = pos2 {
      char == val
    } else {
      false
    };

    if valid1 ^ valid2 {
      valid_count += 1;
    }
  }
  println!("{}", valid_count);

  Ok(())
}
