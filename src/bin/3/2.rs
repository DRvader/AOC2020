use std::error::Error;

fn traverse(mountain: &Vec<Vec<char>>, x_change: usize, y_change: usize) -> usize {
  let mut x_pos = x_change;
  let mut y_pos = y_change;
  let mut tree_count = 0;
  while y_pos < mountain.len() {
    let y = &mountain[y_pos];
    if y[x_pos % y.len()] == '#' {
      tree_count += 1;
    }

    y_pos += y_change;
    x_pos += x_change;
  }

  return tree_count;
}

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("3.txt")?;

  let mut mountain = Vec::new();
  for line in buf.lines() {
    let vec: Vec<_> = line.chars().collect();
    mountain.push(vec);
  }

  let tree_count = traverse(&mountain, 1, 1)
    * traverse(&mountain, 3, 1)
    * traverse(&mountain, 5, 1)
    * traverse(&mountain, 7, 1)
    * traverse(&mountain, 1, 2);
  println!("{}", tree_count);

  Ok(())
}
