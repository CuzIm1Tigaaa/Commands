mod instructions;
mod parser;

use instructions::instructions::InstructionList;

use crate::parser::parser::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut instruction_list = InstructionList::new();
    let mut file = File::open("./test.cheaps").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    parse(&mut instruction_list, &contents);
}
