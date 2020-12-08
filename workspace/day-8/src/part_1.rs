use crate::virtual_machine::{Program, VirtualMachine};

pub fn execute_until_loop(program: &Program) -> i32 {
    let mut virtual_machine = VirtualMachine { execution_context: Default::default(), program};
    let mut already_executed_instructions : Vec<i32> = Vec::new();

    while already_executed_instructions.contains(&virtual_machine.execution_context.pc) == false {
        already_executed_instructions.push(virtual_machine.execution_context.pc);
        virtual_machine.execute_one_cycle();
    }

    virtual_machine.execution_context.acc
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

        let acc = execute_until_loop(&program);

        assert_eq!(acc, 5);
    }
}