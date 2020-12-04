// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

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
      let v = kv[1];

      if !passport_fields.contains(&k) {
        let valid = match k {
          "byr" => {
            let year: u32 = v.parse()?;
            year >= 1920 && year <= 2002
          }
          "iyr" => {
            let year: u32 = v.parse()?;
            year >= 2010 && year <= 2020
          }
          "eyr" => {
            let year: u32 = v.parse()?;
            year >= 2020 && year <= 2030
          }
          "hgt" => {
            let number: Result<f32, _> = v[0..v.len() - 2].parse();
            if let Ok(number) = number {
              match &v[v.len() - 2..] {
                "cm" => number >= 150.0 && number <= 193.0,
                "in" => number >= 59.0 && number <= 76.0,
                _ => false,
              }
            } else {
              false
            }
          }
          "hcl" => {
            let chars: Vec<char> = v.chars().collect();
            if chars.len() < 7 {
              false
            } else {
              let value = chars[0] == '#';

              let mut value2 = true;
              for i in 1..=6 {
                if ![
                  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                ]
                .contains(&chars[i])
                {
                  value2 = false;
                  break;
                }
              }
              value && value2
            }
          }
          "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
          "pid" => {
            v.len() == 9
              && if let Ok(_) = v.parse::<u32>() {
                true
              } else {
                false
              }
          }
          _ => false,
        };

        if valid {
          passport_fields.push(k);
        }
      }
    }
  }
  println!("{}", valid_passports);

  Ok(())
}
