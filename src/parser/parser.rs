use crate::instructions::history::*;

pub fn parse(schema: &mut History, file_content: &str) {
    for (line_no, line) in file_content.lines().enumerate() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            // empty lines and comments shall not be processed
            continue;
        }

        if !line.starts_with("\t") {
            // labels
            let label = line.trim();
            if label.split_whitespace().count() > 1 {
                println!(
                    "Error on line {0}: Label should not have arguments.",
                    line_no + 1
                );
                return;
            }
            print!("Label found: {0}\n", label);
            continue;
        }

        let line = line.trim_start_matches('\t');
        let args: Vec<&str> = line.split_whitespace().collect();

        let op_name = args[0];
        let Some(op) = schema.get_instruction_by_name(op_name) else {
            println!(
                "Error on line {0}:\n  Unknown instruction \"{1}\"",
                line_no + 1,
                op_name
            );
            return;
        };

        let op_code = op.get_op_code();
        print!(
            "Op code: {0}, Arguments: {1}\n",
            op_code,
            format!("{:?}", args[1..].to_vec())
        );

        let (valid, value) = op.parse_args(args[1..].to_vec());
        if !valid {
            println!(
                "Error on line {0}:\n  Invalid arguments for instruction \"{1}\"",
                line_no + 1,
                op_name
            );
            return;
        } else {
            println!("  Arguments are valid.");
            println!("  Encoded: {:032b}", value);
        }

        // let result = op.execute(args);
        // println!("  {0}", result);
    }
}
