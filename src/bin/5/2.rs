use std::error::Error;

fn upper_half(range: (u16, u16)) -> (u16, u16) {
  let mut range = (range.0 as f32, range.1 as f32);
  let inter = (range.1 - range.0) / 2.0;
  range.0 += inter;
  return (range.0.ceil() as u16, range.1 as u16);
}

fn lower_half(range: (u16, u16)) -> (u16, u16) {
  let mut range = (range.0 as f32, range.1 as f32);
  let inter = (range.1 - range.0) / 2.0;
  range.1 -= inter;
  return (range.0 as u16, range.1 as u16);
}

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("5.txt")?;

  let mut seat_ids = vec![1; 127 * 8];
  for line in buf.lines() {
    let mut fb = (0, 127);
    let mut lr = (0, 7);

    for char in line.chars() {
      if char == 'F' {
        fb = lower_half(fb);
      } else if char == 'B' {
        fb = upper_half(fb);
      } else if char == 'L' {
        lr = lower_half(lr);
      } else if char == 'R' {
        lr = upper_half(lr);
      }
    }
    let seat_id = fb.0 * 8 + lr.0;
    seat_ids[seat_id as usize] = 0;
  }
  let mut base_index = 9;
  while seat_ids[base_index] == 1 {
    base_index += 1;
  }
  for (index, i) in seat_ids[base_index..].into_iter().enumerate() {
    if *i == 1 {
      println!("{}", base_index + index);
      break;
    }
  }

  Ok(())
}
