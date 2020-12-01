use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("1.txt")?;

  let mut leger: Vec<u64> = Vec::new();
  for line in buf.lines() {
    let num = line.parse()?;
    if num > 2020 {
      continue;
    }
    leger.push(num);
  }
  leger.sort();

  for outer_i in 0..leger.len() {
    let a = leger[outer_i];
    let target = 2020 - a;
    if let Ok(_) = leger.binary_search(&target) {
      println!("Answer: {}", target * a);
      return Ok(());
    }
  }

  Ok(())
}
