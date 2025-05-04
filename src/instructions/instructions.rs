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
                    println!("Error: Invalid arguments for {} instruction.", $opname);
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

fn get_register_number(register: &str) -> u32 {
    let check_register = Regex::new(REGEX_REGISTER).unwrap();
    if let Some(capture) = check_register.captures(register) {
        if let Some(rgx_match) = capture.get(1) {
            return rgx_match.as_str().parse::<u32>().unwrap_or(16);
        }
    }
    16
}

fn parse_nope(args: Vec<&str>) -> (bool, u32) {
    if args.len() != 0 {
        return (false, 0);
    }
    (true, 0)
}

fn parse_let(args: Vec<&str>) -> (bool, u32) {
    if args.len() != 3 || args[1] != "BE" {
        return (false, 0);
    }

    let register = get_register_number(args[0]);
    if register > 15 {
        println!("Error: Register number out of range (0-15).");
        return (false, 0);
    }

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
    if summand_1 > 15 || summand_2 > 15 || result > 15 {
        println!("Error: Register number out of range (0-15).");
        return (false, 0);
    }

    let carry = args.windows(2).any(|pair| pair == ["WITH", "CARRY"]);
    let update = args.windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

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

    let update = args.windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

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
    if multiplicand > 15 || multiplier > 15 || result > 15 {
        println!("Error: Register number out of range (0-15).");
        return (false, 0);
    }

    let update = args.windows(2).any(|pair| pair == ["WITH", "UPDATE"]);

    let mut binary = (update as u32) << 14;
    binary |= ((multiplicand as u32) << 23) | ((multiplier as u32) << 19) | ((result as u32) << 15);
    (true, binary)
}

define_instruction!(Nope, 0b0, "NOPE", parse_nope);
define_instruction!(Let, 0b00001, "LET", parse_let);
define_instruction!(Add, 0b01011, "ADD", parse_add);
define_instruction!(Sub, 0b01100, "SUB", parse_sub);
define_instruction!(Mul, 0b01101, "MUL", parse_mul);
define_instruction!(Div, 0b01110, "DIV", parse_div); // TODO: Implement DIV instruction

// 0000 0000 1000 0011 1110 1000 0000 0000
// 0000 1000 1000 0011 1110 1000 0000 0000
