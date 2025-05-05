use regex::Regex;

const REGEX_REGISTER: &str = r"^R([0-9]{1,2})$";

macro_rules! define_instruction {
    ($name:ident, $opcode:expr, $opname:expr, $opparser:path) => {
        #[derive(Debug)]
        pub struct $name;

        impl Instruction for $name {
            fn get_op_code(&self) -> u8 {
                $opcode
            }
            fn get_op_name(&self) -> &str {
                $opname
            }
            fn parse_args(&self, args: Vec<&str>) -> (bool, u32) {
                let (valid, value) = $opparser(args);
                if !valid {
                    return (false, 0);
                }
                return (true, (($opcode as u32) << 27) | (value as u32));
            }
        }
    };
}

pub trait Instruction: std::fmt::Debug {
    fn get_op_code(&self) -> u8;
    fn get_op_name(&self) -> &str;
    fn parse_args(&self, args: Vec<&str>) -> (bool, u32);
}

pub struct InstructionList {
    instructions: Vec<Box<dyn Instruction>>,
}

impl InstructionList {
    pub fn new() -> Self {
        Self {
            instructions: vec![
                Box::new(Nope), Box::new(Let), Box::new(Copy), Box::new(Read), Box::new(Write),
                Box::new(Push), Box::new(Pop), Box::new(Peek), Box::new(Jump), Box::new(Eval),
                Box::new(Comp), Box::new(Add), Box::new(Sub), Box::new(Mul), Box::new(Div),
                Box::new(Neg), Box::new(Or), Box::new(And), Box::new(Xor), Box::new(Shift),
                Box::new(Log), Box::new(Clear), Box::new(SetPx),
            ],            
        }
    }

    pub fn get_instruction_by_name(&mut self, name: &str) -> Option<&Box<dyn Instruction>> {
        self.instructions.iter().find(|x| x.get_op_name().eq(name))
    }
}

fn get_register_number(register: &str) -> u32 {
    let check_register = Regex::new(REGEX_REGISTER).unwrap();
    if let Some(capture) = check_register.captures(register) {
        if let Some(rgx_match) = capture.get(1) {
            let result = rgx_match.as_str().parse::<u32>().unwrap_or(16);
            if result < 15 {
                return result;
            }
        }
    }
    panic!("{0} is not a valid register!", register);
}

fn parse_nope(args: Vec<&str>) -> (bool, u32) {
    if args.len() != 0 {
        panic!("The NOPE instruction requires no further arguments!")
    }
    (true, 0)
}

fn parse_let(args: Vec<&str>) -> (bool, u32) {
    if args.len() != 3 || args[1] != "BE" {
        return (false, 0);
    }

    let register = get_register_number(args[0]);
    let value = args[2].parse::<u32>().unwrap_or(0);
    let binary = ((register as u32) << 23) | ((value as u32) << 7);
    (true, binary)
}

fn parse_add(args: Vec<&str>) -> (bool, u32) {
    if args.len() < 5 || args[1] != "TO" || args[3] != "INTO" {
        return (false, 0);
    }

    let summand_1 = get_register_number(args[0]);
    let summand_2 = get_register_number(args[2]);
    let result = get_register_number(args[4]);
    let carry = args[6..].windows(2).any(|pair| pair == ["WITH", "CARRY"]);
    let update = args[6..].windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

    let mut binary = (((carry as u32) << 1) | (update as u32)) << 13;
    binary |= ((summand_1 as u32) << 23) | ((summand_2 as u32) << 19) | ((result as u32) << 15);
    (true, binary)
}

fn parse_sub(args: Vec<&str>) -> (bool, u32) {
    if args.len() < 5 || args[1] != "FROM" || args[3] != "INTO" {
        return (false, 0);
    }

    let minuend = get_register_number(args[0]);
    let subtrahend = get_register_number(args[2]);
    let result = get_register_number(args[4]);
    if minuend > 15 || subtrahend > 15 || result > 15 {
        println!("Error: Register number out of range (0-15).");
        return (false, 0);
    }

    let update = args.len() == 9 && args[6..8].windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

    let mut binary = (update as u32) << 14;
    binary |= ((minuend as u32) << 23) | ((subtrahend as u32) << 19) | ((result as u32) << 15);
    (true, binary)
}

fn parse_mul(args: Vec<&str>) -> (bool, u32) {
    if args.len() < 5 || args[1] != "WITH" || args[3] != "INTO" {
        return (false, 0);
    }

    let multiplicand = get_register_number(args[0]);
    let multiplier = get_register_number(args[2]);
    let result = get_register_number(args[4]);
    if multiplicand > 15 || multiplier > 15 || result > 15 {
        println!("Error: Register number out of range (0-15).");
        return (false, 0);
    }

    let update = args.windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

    let mut binary = (update as u32) << 14;
    binary |= ((multiplicand as u32) << 23) | ((multiplier as u32) << 19) | ((result as u32) << 15);
    (true, binary)
}

fn parse_div(args: Vec<&str>) -> (bool, u32) {
    if args.len() < 5 || args[1] != "BY" || args[3] != "INTO" {
        return (false, 0);
    }

    let multiplicand = get_register_number(args[0]);
    let multiplier = get_register_number(args[2]);
    let result = get_register_number(args[4]);
    let update = args.windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

    let mut binary = (update as u32) << 14;
    binary |= ((multiplicand as u32) << 23) | ((multiplier as u32) << 19) | ((result as u32) << 15);
    (true, binary)
}

fn parse_log(args: Vec<&str>) -> (bool, u32) {
    if args.len() != 1 {
        panic!("The LOG instruction only requires one argument!");
    }

    let register = get_register_number(args[0]);
    let binary = register << 23;
    (true, binary)
}

define_instruction!(Nope,   0b00000,    "NOPE",    parse_nope);
define_instruction!(Let,    0b00001,    "LET",     parse_let);
define_instruction!(Copy,   0b00010,    "COPY",    parse_nope);
define_instruction!(Write,  0b00011,    "WRITE",   parse_nope);
define_instruction!(Read,   0b00100,    "READ",    parse_nope);
define_instruction!(Push,   0b00101,    "PUSH",    parse_nope);
define_instruction!(Pop,    0b00110,    "POP",     parse_nope);
define_instruction!(Peek,   0b00111,    "PEEK",    parse_nope);
define_instruction!(Jump,   0b01000,    "JUMP",    parse_add);
define_instruction!(Eval,   0b01001,    "EVAL",    parse_add);
define_instruction!(Comp,   0b01010,    "COMP",    parse_add);
define_instruction!(Add,    0b01011,    "ADD",     parse_add);
define_instruction!(Sub,    0b01100,    "SUB",     parse_sub);
define_instruction!(Mul,    0b01101,    "MUL",     parse_mul);
define_instruction!(Div,    0b01110,    "DIV",     parse_div);
define_instruction!(Neg,    0b01111,    "NEG",     parse_div);
define_instruction!(Or,     0b10000,    "OR",      parse_div);
define_instruction!(And,    0b10001,    "AND",     parse_div);
define_instruction!(Xor,    0b10010,    "XOR",     parse_div);
define_instruction!(Shift,  0b10011,    "SHIFT",   parse_div);
define_instruction!(Log,    0b10100,    "LOG",     parse_log);
define_instruction!(Clear,  0b10101,    "CLEAR",   parse_div);
define_instruction!(SetPx,  0b10110,    "SETPX",   parse_div);
