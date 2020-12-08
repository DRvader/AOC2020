use std::{any::Any, collections::HashSet, error::Error};

#[derive(Copy, Clone)]
pub enum Instr {
  NOP(i64),
  ACC(i64),
  JMP(i64),
}

#[derive(Debug)]
pub enum ProgramResult {
  Success(Box<dyn Any>),
  Failure(Box<dyn Any>),
}

#[derive(Clone)]
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
      "nop" => Instr::NOP(args[0].parse()?),
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

  pub fn run(&mut self) -> ProgramResult {
    let mut pc_values = HashSet::new();
    loop {
      let instr = &self.instructions[self.pc];
      if pc_values.contains(&self.pc) {
        break ProgramResult::Failure(Box::new(pc_values));
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
        Instr::NOP(_) => {
          self.pc += 1;
        }
      }

      if self.pc >= self.instructions.len() {
        break ProgramResult::Success(Box::new(self.acc));
      }
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let buf = AOC2020::read_datafile("8.txt")?;

  let mut initial_machine = Machine::new();
  initial_machine.parse_str(buf)?;
  let initial_machine = initial_machine;
  let mut machine = initial_machine.clone();
  let fail = if let ProgramResult::Failure(fail) = machine.run() {
    fail
  } else {
    unreachable!();
  };
  let fail: &HashSet<usize> = fail.downcast_ref().unwrap();
  for i in fail {
    let mut new_machine = initial_machine.clone();
    match machine.instructions[*i] {
      Instr::JMP(arg) => {
        new_machine.instructions[*i] = Instr::NOP(arg);
      }
      Instr::NOP(arg) => {
        new_machine.instructions[*i] = Instr::JMP(arg);
      }
      _ => continue,
    }
    if let ProgramResult::Success(acc) = new_machine.run() {
      println!("{}", acc.downcast_ref::<i64>().unwrap());
    }
  }

  Ok(())
}
