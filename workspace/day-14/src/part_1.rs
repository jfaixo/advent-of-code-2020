use crate::virtual_machine::{Program, Operation, ExecutionContext};

fn simulate(program: &Program) -> ExecutionContext {
    let mut context : ExecutionContext = Default::default();

    for instruction in program {
        match instruction {
            Operation::Mask(mask) => {
                context.mask = *mask;
            }
            Operation::Write(write) => {
                // Apply the mask
                let mut value = write.value | context.mask.one_mask;
                value = value & !context.mask.zero_mask;
                context.memory.insert(write.address, value);
            }
        }
    }

    context
}

pub fn sum_of_all_mem_values_part_1(program: &Program) -> u64 {
    simulate(program)
        .memory
        .values()
        .map(|value| {
            *value
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn example_case() {
        let content = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
".to_string();

        let program = parse_string(content);
        let result = sum_of_all_mem_values_part_1(&program);
        assert_eq!(result, 165);
    }
}