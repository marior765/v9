mod analyzer;
mod file_system;
mod types;
use analyzer::Analyzer;
use file_system::reader::Reader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn multi_concurrency() {
    let reader = Arc::new(Mutex::new(Reader::init()));
    let reader_c = Arc::clone(&reader);
    let read_handler = thread::spawn(move || {
        reader_c
            .lock()
            .unwrap()
            .deep_read("index.js")
            .expect("File not found");
    });
    read_handler.join().unwrap();
    let result = reader.lock().unwrap().return_chars();
    let mut analyzer = Analyzer::new(&result);
    analyzer.analyze();
    println!("{:?}", analyzer.buffer);
    for i in analyzer.lexer {
        println!("{:?}", i);
    }
}

// fn single_concurrency() {
//     let mut compiler = Reader::init();
//     compiler.read_file("index.js").expect("File not found");
//     compiler.show();
// }

fn main() {
    let now = Instant::now();
    multi_concurrency();
    println!("{}", now.elapsed().as_secs_f64());
}

// 0.0005481992650000002 - single
// 0.0006300508879999987 - multi
