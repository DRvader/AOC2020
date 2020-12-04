use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("4.txt")? + "\n";

  let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  let mut passport_fields = Vec::new();
  let mut valid_passports = 0;
  for line in buf.lines() {
    if line.len() == 0 {
      if passport_fields.len() == required_fields.len() {
        valid_passports += 1;
      }
      passport_fields.clear();
      continue;
    }

    let fields = line.split_whitespace();
    for field in fields {
      let kv: Vec<&str> = field.split_terminator(':').collect();
      let k = kv[0];

      if required_fields.contains(&k) && !passport_fields.contains(&k) {
        passport_fields.push(k);
      }
    }
  }
  println!("{}", valid_passports);

  Ok(())
}
