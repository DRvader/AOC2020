use std::{collections::HashSet, error::Error};

pub enum Instr {
  NOP,
  ACC(i64),
  JMP(i64),
}

struct Machine {
  pub acc: i64,
  pub pc: usize,
  pub instructions: Vec<Instr>,
}

impl Machine {
  pub fn new() -> Machine {
    Machine {
      acc: 0,
      pc: 0,
      instructions: Vec::new(),
    }
  }

  fn parse_instr(&self, instr: &str, args: &[&str]) -> Result<Instr, Box<dyn Error>> {
    Ok(match instr {
      "jmp" => Instr::JMP(args[0].parse()?),
      "acc" => Instr::ACC(args[0].parse()?),
      "nop" => Instr::NOP,
      _ => unreachable!(),
    })
  }

  pub fn parse_str<S: AsRef<str>>(&mut self, file: S) -> Result<(), Box<dyn Error>> {
    let file: &str = file.as_ref();
    for line in file.lines() {
      let line: Vec<&str> = line.split_whitespace().collect();
      self
        .instructions
        .push(self.parse_instr(line[0], &line[1..])?);
    }

    Ok(())
  }

  pub fn run(&mut self) -> i64 {
    let mut pc_values = HashSet::new();
    loop {
      let instr = &self.instructions[self.pc];
      if pc_values.contains(&self.pc) {
        break self.acc;
      } else {
        pc_values.insert(self.pc);
      }

      match instr {
        Instr::JMP(arg) => {
          let arg = *arg;
          if arg < 0 {
            let arg = -arg as usize;
            if arg > self.pc {
              self.pc = 0;
            } else {
              self.pc -= arg;
            }
          } else {
            let arg = arg as usize;
            self.pc += arg;
          }
        }
        Instr::ACC(arg) => {
          self.acc += arg;
          self.pc += 1;
        }
        Instr::NOP => {
          self.pc += 1;
        }
      }

      if self.pc > self.instructions.len() {
        self.pc = self.instructions.len() - 1;
      }
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("8.txt")?;

  let mut machine = Machine::new();
  machine.parse_str(buf)?;
  println!("{}", machine.run());

  Ok(())
}
