#![allow(non_snake_case)]

use std::{
  error::Error,
  fs::File,
  io::Read,
  path::{Path, PathBuf},
};

pub fn get_datadir() -> Result<PathBuf, Box<dyn Error>> {
  Ok(
    std::env::current_exe()?
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("data"),
  )
}

pub fn read_datafile<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
  let mut file = File::open(get_datadir()?.join(path))?;
  let mut buf = String::new();
  file.read_to_string(&mut buf)?;
  return Ok(buf);
}
