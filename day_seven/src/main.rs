//Function which reads the file and returns the contents of the file
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file(path: &str) -> String {
    let file = File::open(path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

fn main() {
    println!("Hello, world!");
}
