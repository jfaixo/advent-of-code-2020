use std::fs;
use crate::virtual_machine::{Program, Instruction, Opcode, opcode_from_str};

pub fn parse_text_file(file_name: String) -> Program {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Program {
    let mut program : Program = Default::default();

    let parts : Vec<&str> = content
        // Split all opcodes and operands
        .split_ascii_whitespace()
        .collect();
    parts.chunks(2).for_each(|instruction|{
        let opcode : Opcode = opcode_from_str(instruction[0]);
        let operand : i32 = instruction[1].parse().expect("Invalid operand");
        program.instructions.push(Instruction{
            opcode,
            operand,
        })
    });
    
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
".to_string();
        let program = parse_string(content);

        assert_eq!(program.instructions[0], Instruction{ opcode: Opcode::NOP, operand: 0 });
        assert_eq!(program.instructions[1], Instruction{ opcode: Opcode::ACC, operand: 1 });
        assert_eq!(program.instructions[2], Instruction{ opcode: Opcode::JMP, operand: 4 });
        assert_eq!(program.instructions[3], Instruction{ opcode: Opcode::ACC, operand: 3 });
        assert_eq!(program.instructions[4], Instruction{ opcode: Opcode::JMP, operand: -3 });
        assert_eq!(program.instructions[5], Instruction{ opcode: Opcode::ACC, operand: -99 });
        assert_eq!(program.instructions[6], Instruction{ opcode: Opcode::ACC, operand: 1 });
        assert_eq!(program.instructions[7], Instruction{ opcode: Opcode::JMP, operand: -4 });
        assert_eq!(program.instructions[8], Instruction{ opcode: Opcode::ACC, operand: 6 });
    }
}