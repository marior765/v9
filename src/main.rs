mod analyzer;
mod file_system;
mod types;
use analyzer::Analyzer;
use file_system::compiler::Compiler;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

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
    read_handler.join().unwrap();
    let mut analyzer = Analyzer::new(compiler.lock().unwrap().completed_buf());
    for el in &analyzer.storage {
        println!("{:?}", el);
    }
}

// fn single_concurrency() {
//     let mut compiler = Compiler::init();
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
