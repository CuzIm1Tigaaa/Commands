#[cfg(test)]
mod tests {

    use crate::instructions::instructions::*;

    fn format_binary_with_spaces(n: u32) -> String {
        let bin_str = format!("{:032b}", n);
        bin_str
            .chars()
            .rev() // reverse to group from the right
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .rev() // reverse back
            .collect()
    }

    #[test]
    fn test_instruction_let_correct_parsed() {
        let let_instr = Let;
        // LET R1 BE 2000
        let args = vec!["R1", "BE", "2000"];
        let (valid, value) = let_instr.parse_args(args);

        assert!(valid == true);
        let bin_str = format_binary_with_spaces(value);
        let expected = format_binary_with_spaces(0b00001000100000111110100000000000); // 2000 in binary
        assert_eq!(expected, bin_str);
    }

    macro_rules! add_instruction_parser_test {
        ($($name:ident, $args:expr, $expected:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let add_instr = Add;
					let args: Vec<&str> = $args;
					let (valid, value) = add_instr.parse_args(args);

					assert!(valid == true);
					let bin_str = format_binary_with_spaces(value);
					assert_eq!(format_binary_with_spaces($expected), bin_str);
				}
			)*
		}
	}

    add_instruction_parser_test! {
        test_instruction_add_without_flags, vec!["R1", "TO", "R15", "INTO", "R2"], 0b01011000111110010000000000000000,
        test_instruction_add_with_carry, vec!["R1", "TO", "R15", "INTO", "R2", "WITH", "CARRY"], 0b01011000111110010100000000000000,
        test_instruction_add_with_update, vec!["R1", "TO", "R15", "INTO", "R2", "WITH", "UPDATE"], 0b01011000111110010010000000000000,
        test_instruction_add_with_carry_and_update, vec!["R1", "TO", "R15", "INTO", "R2", "WITH", "CARRY", "WITH", "UPDATE"], 0b01011000111110010110000000000000,
        test_instruction_add_with_carry_and_update_2, vec!["R1", "TO", "R15", "INTO", "R2", "WITH", "UPDATE", "WITH", "CARRY"], 0b01011000111110010110000000000000,
    }

    macro_rules! sub_instruction_parser_test {
        ($($name:ident, $args:expr, $expected:expr,)*) => {
			$(
				#[test]
				fn $name() {
					let sub_instr = Sub;
					let args: Vec<&str> = $args;
					let (valid, value) = sub_instr.parse_args(args);

					assert!(valid == true);
					let bin_str = format_binary_with_spaces(value);
					assert_eq!(format_binary_with_spaces($expected), bin_str);
				}
			)*
		}
	}

    sub_instruction_parser_test! {
        test_instruction_sub_without_flags, vec!["R1", "FROM", "R15", "INTO", "R2"], 0b01100000111110010000000000000000,
        test_instruction_sub_with_update, vec!["R1", "FROM", "R15", "INTO", "R2", "WITH", "UPDATE"], 0b01100000111110010100000000000000,
    }
}
