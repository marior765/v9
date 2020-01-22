use super::module::Module;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct Compiler {
  buffer: HashMap<String, Module>,
  deep: i8,
  // callstack: Option<CallStack>,
}

impl Compiler {
  pub fn init() -> Self {
    Compiler {
      buffer: HashMap::new(),
      deep: 0,
      // callstack: None,
    }
  }

  pub fn read_file(&mut self, file_path: &str) -> std::io::Result<()> {
    self.deep += 1;
    let path = Path::new(file_path);
    let file_name = path
      .file_name() /* -_- */
      .expect("Nameless")
      .to_str()
      .unwrap();
    self
      .buffer
      .insert(String::from(file_name), Module::default());
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf);
    for line in buf.split("\n") {
      // line_counter += 1;
      if line.contains("import ") {
        let splitted: Vec<&str> = line.split("from ").collect();
        let mut path_next =
          splitted[splitted.len() - 1].replace(&['(', ')', ',', '\"', ';', ':', '\''][..], "");
        path_next.push_str(".js");
        self.read_file(path_next.as_ref()).expect("File not found");
      }
    }
    // let mut line_counter = 0;
    if let Some(module) = self.buffer.get_mut(file_name) {
      //self.parse(buf.split_ascii_whitespace().collect());
      // let mut column_counter = 1;
      // for elem in buf.split_ascii_whitespace() {
      //     let elem = elem.trim();
      //     if elem.len() != 0 {
      //         module.scheme.push(Element {
      //             value: elem.into(),
      //             r#type: analyze(elem),
      //             position: Position {
      //                 line: line_counter,
      //                 column: column_counter as u8,
      //             },
      //         });
      //         column_counter += elem.len() + 1;
      //     }
      // }
      module.content.push(buf);
    }
    Ok(())
  }

  pub fn show(&self) {
    println!("Deep: {}", self.deep);
    for (key, val) in self.buffer.iter() {
      println!("File: {}", key);
      println!("Content:");
      for content in val.content.iter() {
        println!("{}", content);
      }
      println!("Scheme:");
      for scheme in val.scheme.iter() {
        println!("{}", scheme);
      }
    }
  }
}
