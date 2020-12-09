use std::{collections::VecDeque, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("9.txt")?;
  let nums: Vec<usize> = buf.lines().map(|l| l.parse().unwrap()).collect();

  let preamble_size = 25;
  let mut summed_queue = VecDeque::with_capacity(preamble_size * preamble_size);
  let mut queue = VecDeque::with_capacity(preamble_size);
  for i in 0..preamble_size {
    queue.push_back(nums[i]);
    for x in 0..preamble_size {
      if i == x {
        continue;
      }
      summed_queue.push_back(nums[i] + nums[x]);
    }
  }

  for num in &nums[preamble_size..] {
    if !summed_queue.contains(num) {
      println!("{}", num);
      return Ok(());
    }
    summed_queue.drain(0..preamble_size - 1);
    queue.pop_front();
    summed_queue.extend(queue.iter().map(|n| n + num));
    queue.push_back(*num);
  }

  Ok(())
}
