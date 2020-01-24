use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct Reader {
  buffer: Vec<String>,
  deep: i8,
}

impl Reader {
  pub fn init() -> Self {
    Reader {
      buffer: Vec::new(),
      deep: 0,
    }
  }

  pub fn read_file(&mut self, file_path: &str) -> std::io::Result<String> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf);
    Ok(buf)
  }

  pub fn deep_read<'a>(&mut self, file_path: &str) -> std::io::Result<()> {
    self.deep += 1;
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    println!("File size: {:?}", reader.read_to_string(&mut buf));
    for line in buf.lines() {
      if line.contains("import ") {
        let splitted: Vec<&str> = line.split("from ").collect();
        let mut path =
          splitted[splitted.len() - 1].replace(&['(', ')', ',', '\"', ';', ':', '\''][..], "");
        path.push_str(".js");
        self.deep_read(path.as_ref()).expect("File not found");
      }
    }
    self.buffer.push(buf);
    Ok(())
  }

  // pub fn show(&self) {
  //   println!("Deep: {}", self.deep);
  //   for (key, val) in self.buffer.iter() {
  //     println!("File: {}", key);
  //     println!("Content:");
  //     for content in val.content.iter() {
  //       println!("{:?}", val.content);
  //     }
  //     // println!("Scheme:");
  //     // for scheme in val.scheme.iter() {
  //     //   println!("{}", scheme);
  //     // }
  //   }
  // }

  pub fn return_chars(&self) -> String {
    self
      .buffer
      // .iter()
      // .map(|v| v.split("\n").filter(|l| !l.contains("from ")).collect())
      // .collect::<Vec<String>>()
      .concat()
  }
}
