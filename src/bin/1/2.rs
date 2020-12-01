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
      for inner_inner_i in outer_i..leger.len() {
        let num = leger[outer_i] + leger[inner_i] + leger[inner_inner_i];
        if num == 2020 {
          println!(
            "Answer: {}",
            leger[outer_i] * leger[inner_i] * leger[inner_inner_i]
          );
          return Ok(());
        }

        if num > 2020 {
          break;
        }
      }
    }
  }

  Ok(())
}
