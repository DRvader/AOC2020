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
    for inner_i in outer_i..leger.len() {
      let a = leger[outer_i];
      let b = leger[inner_i];
      let mut target = 2020 - a;
      if b > target {
        break;
      }
      target -= b;
      if let Ok(_) = leger.binary_search(&target) {
        println!("Answer: {}", target * a * b);
        return Ok(());
      }
    }
  }

  Ok(())
}
