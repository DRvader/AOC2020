use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("3.txt")?;

  let mut x_pos = 0;
  let mut tree_count = 0;
  for line in buf.lines().skip(1) {
    x_pos += 3;
    if line.chars().nth(x_pos % line.chars().count()).unwrap() == '#' {
      tree_count += 1;
    }
  }
  println!("{}", tree_count);

  Ok(())
}
