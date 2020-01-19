mod analyzer;
mod stack;
use analyzer::{analyze, Syntax};

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Debug)]
struct Position {
    line: u8,
    column: u8,
}

#[derive(Debug)]
struct Element {
    value: String,
    r#type: Syntax,
    position: Position,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // "Element {{ \n value: '{}'\n type: {:?}\n position: {:?} \n}}",
            "Element {{  value: '{}' | type: {:?} | position: {:?} }}",
            self.value,
            self.r#type,
            self.position
        )
    }
}

#[derive(Debug, Default)]
struct Module {
    content: Vec<String>,
    scheme: Vec<Element>,
}

#[derive(Debug)]
struct Compiler {
    buffer: HashMap<String, Module>,
    deep: i8,
}

impl Compiler {
    fn init() -> Self {
        Compiler {
            buffer: HashMap::new(),
            deep: 0,
        }
    }

    // fn read_line(&self, line: String) -> Vec<Element> {
    //     line
    //         .split_ascii_whitespace()
    //         .map(|elem| Element {
    //             value: elem.into(),
    //             r#type: self.analyze(elem),
    //             position: Position { line: 0, column: 0 },
    //         })
    //         .collect()
    // }

    fn read_file(&mut self, file_path: &str) -> std::io::Result<()> {
        self.deep += 1;
        let path = Path::new(file_path);
        let file_name = path.file_name().expect("Nameless").to_str().unwrap();
        self.buffer
            .insert(String::from(file_name), Module::default());
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut line_counter = 0;
        for line in reader.lines() {
            line_counter += 1;
            let current_line = line?.clone();
            if current_line.contains("import ") {
                let splitted: Vec<&str> = current_line.split("from ").collect();
                let mut path_next = splitted[splitted.len() - 1]
                    .replace(&['(', ')', ',', '\"', ';', ':', '\''][..], "");
                path_next.push_str(".js");
                self.read_file(path_next.as_ref()).expect("File not found");
            }
            if let Some(module) = self.buffer.get_mut(file_name) {
                let mut column_counter = 1;
                for elem in current_line.split_ascii_whitespace() {
                    module.scheme.push(Element {
                        value: elem.into(),
                        r#type: analyze(elem),
                        position: Position {
                            line: line_counter,
                            column: column_counter as u8,
                        },
                    });
                    column_counter += elem.len() + 1;
                }
                module.content.push(current_line);
            }
        }
        Ok(())
    }

    fn show(&self) {
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

fn multi_concurrency() {
    let compiler = Arc::new(Mutex::new(Compiler::init()));
    let compiler_c = Arc::clone(&compiler);
    let read_handler = thread::spawn(move || {
        compiler_c
            .lock()
            .unwrap()
            .read_file("index.js")
            .expect("File not found");
    });
    // let compiler = Arc::clone(&compiler);
    // let show_handler = thread::spawn(move || {
    //     compiler.lock().unwrap().show();
    // });
    read_handler.join().unwrap();
    // show_handler.join().unwrap();
    compiler.lock().unwrap().show();
}

fn single_concurrency() {
    let mut compiler = Compiler::init();
    compiler.read_file("index.js").expect("File not found");
    compiler.show();
}

fn main() {
    let now = Instant::now();
    // single_concurrency();
    multi_concurrency();
    println!("{}", now.elapsed().as_secs_f64());
}

// 0.0005481992650000002 - single
// 0.0006300508879999987 - multi
