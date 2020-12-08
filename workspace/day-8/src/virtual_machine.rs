

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Opcode {
    NOP,
    JMP,
    ACC
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: i32
}

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>
}

#[derive(Debug, Default)]
pub struct ExecutionContext {
    pub pc: i32,
    pub acc: i32
}

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    pub execution_context: ExecutionContext,
    pub program: &'a Program,
}

impl <'a> VirtualMachine<'a> {
    pub fn execute_one_cycle(&mut self) {
        match self.program.instructions[self.execution_context.pc as usize].opcode {
            Opcode::NOP => {
                self.execution_context.pc += 1;
            }
            Opcode::JMP => {
                self.execution_context.pc += self.program.instructions[self.execution_context.pc as usize].operand;
            }
            Opcode::ACC => {
                self.execution_context.acc += self.program.instructions[self.execution_context.pc as usize].operand;
                self.execution_context.pc += 1;
            }
        }
    }
}

pub fn opcode_from_str(str: &str) -> Opcode {
    match str {
        "nop" => Opcode::NOP,
        "jmp" => Opcode::JMP,
        "acc" => Opcode::ACC,
        _ => panic!("Unknown opcode")
    }
}
