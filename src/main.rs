mod instructions;
mod parser;

use crate::instructions::history::*;
use crate::parser::parser::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut schema = History::new();
    let mut file = File::open("./test.cheaps").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    parse(&mut schema, &contents);
}
