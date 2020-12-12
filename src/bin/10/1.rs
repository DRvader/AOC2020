use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("10.txt")?;

  let mut adapters: Vec<usize> = buf.lines().map(|l| l.parse().unwrap()).collect();
  adapters.sort();

  let mut last = 0;
  let mut diff1 = 0;
  let mut diff3 = 1; // The target is three larger than the largest adapter.
  for i in 0..adapters.len() {
    let diff = adapters[i] - last;
    if diff == 1 {
      diff1 += 1;
    } else if diff == 3 {
      diff3 += 1;
    }
    last = adapters[i];
  }
  println!("{}", diff1 * diff3);

  Ok(())
}
