use crate::virtual_machine::{Program, VirtualMachine, Opcode};

pub fn repair_boot(program: &Program) -> i32 {
    // Loop and try to fix instructions until we find the right one
    for i in 0..program.instructions.len() {
        let mut fixed_program  = (*program).clone();

        // If this is a NOP or JMP, fix the program, else skip this instruction
        if program.instructions[i].opcode == Opcode::NOP {
            fixed_program.instructions[i].opcode = Opcode::JMP;
        }
        else if program.instructions[i].opcode == Opcode::JMP {
            fixed_program.instructions[i].opcode = Opcode::NOP;
        }
        else {
            continue
        }

        // Boot the program and look if it loops or if it ends properly
        let mut virtual_machine = VirtualMachine { execution_context: Default::default(), program: &fixed_program};
        let mut already_executed_instructions : Vec<i32> = Vec::new();

        // Same check as in part_1, if we re execute an instruction, then it is an infinite loop, stop
        while already_executed_instructions.contains(&virtual_machine.execution_context.pc) == false {
            already_executed_instructions.push(virtual_machine.execution_context.pc);
            virtual_machine.execute_one_cycle();

            // If the PC is at the end of boot code, we found the fix
            if virtual_machine.execution_context.pc == program.instructions.len() as i32 {
                return virtual_machine.execution_context.acc;
            }
        }
    }

    return 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn test_part_1_full() {
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

        let acc = repair_boot(&program);

        assert_eq!(acc, 8);
    }
}