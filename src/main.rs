use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct Compiler {
    buffer: HashMap<String, Vec<String>>,
}

impl Compiler {
    fn init() -> Self {
        Compiler {
            buffer: HashMap::new(),
        }
    }

    fn read(&mut self, file_path: &str) -> std::io::Result<()> {
        let path = Path::new(file_path);
        let file_name = path.file_name().expect("Nameless").to_str().unwrap();
        self.buffer.insert(String::from(file_name), Vec::new());
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let current_line = line?.clone();
            if current_line.contains("./") {
                let splitted: Vec<&str> = current_line.split("from ").collect();
                let path_next = splitted[splitted.len() - 1]
                    .replace(&['(', ')', ',', '\"', ';', ':', '\''][..], "");
                self.read(path_next.as_ref()).expect("File not found");
            }
            if let Some(x) = self.buffer.get_mut(file_name) {
                x.push(current_line);
            }
        }
        Ok(())
    }
}

fn main() {
    let mut container = Compiler::init();
    container.read("index.js").expect("File not found");
    for (key, val) in container.buffer.iter() {
        println!("{}: {:?}", key, val);
    }
}
