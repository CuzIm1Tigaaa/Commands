use std::collections::LinkedList;

use crate::instructions::instructions::*;

pub struct History {
    instructions: LinkedList<Box<dyn Instruction>>,
    index: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            instructions: LinkedList::new(),
            index: 0,
        }
    }

    pub fn add_instruction(&mut self, instr: Box<dyn Instruction>) {
        self.instructions.push_back(instr);
    }

    // pub fn get_next_instruction(&mut self) -> Option<Box<dyn Instruction>> {
    //     self.instructions.front();
    // }

    pub fn get_instruction_by_name(&self, op_name: &str) -> Option<&Box<dyn Instruction>> {
        self.instructions
            .iter()
            .find(|op| op.get_op_name() == op_name)
    }

    // pub fn get_instruction_by_code(&self, op_code: u8) -> Option<&Box<dyn Instruction>> {
    //     self.instructions
    //         .iter()
    //         .find(|op| op.get_op_code() == op_code)
    // }

    // pub fn execute(&mut self, op_code: u8, args: &str) {
    //     if let Some(op) = self.get_instruction_by_code(op_code) {
    //         println!("{0}", op.execute(args));
    //         return;
    //     }

    //     println!("There is no instruction with op code {0}", op_code);
    // }
}
