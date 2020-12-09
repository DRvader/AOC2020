use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("9.txt")?;
  let nums: Vec<usize> = buf.lines().map(|l| l.parse().unwrap()).collect();
  let target: usize = 22477624;

  let mut root = 0;
  let mut index = 0;
  let mut sum = 0;
  loop {
    if sum > target {
      root += 1;
      index = root;
      sum = 0;
    } else if sum == target {
      let range = &nums[root..index];
      println!(
        "{}",
        range.iter().min().unwrap() + range.iter().max().unwrap()
      );
      return Ok(());
    }
    sum += nums[index];
    index += 1;
  }
}
