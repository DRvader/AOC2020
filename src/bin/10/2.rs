use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("10.txt")?;

  let mut adapters: Vec<usize> = buf.lines().map(|l| l.parse().unwrap()).collect();
  adapters.push(0);
  adapters.sort();
  let adapters = adapters;

  fn jump(
    adapters: &Vec<usize>,
    cache: &mut Vec<(bool, usize)>,
    base: usize,
    jolts: usize,
    target: usize,
  ) -> (bool, usize) {
    let mut output = false;
    let mut valid_paths = 0;
    for size in 1..=3 {
      let index = base + size;
      println!("{}", adapters[index]);
      if adapters[index] == target {
        output = true;
        valid_paths += 1;
        break;
      }
      if adapters[index] - jolts > 3 {
        break;
      } else {
        if cache[index].0 {
          output = true;
          valid_paths += cache[index].1 + 1;
        } else {
          let result = jump(adapters, cache, index, adapters[index], target);
          if result.0 {
            output = true;
            valid_paths += result.1;
            cache[index] = (true, result.1);
          }
        }
      }
    }
    return (output, valid_paths);
  };

  let target = adapters[adapters.len() - 1];
  let mut success_path = vec![(false, 0); adapters.len()];
  println!("{}", jump(&adapters, &mut success_path, 0, 0, target).1);

  Ok(())
}
