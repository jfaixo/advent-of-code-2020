use crate::virtual_machine::{Program, Operation, WriteOperation, ExecutionContext};

fn simulate(program: &Program) -> ExecutionContext {
    let mut context : ExecutionContext = Default::default();

    for instruction in program {
        match instruction {
            Operation::Mask(mask) => {
                context.mask = *mask;
            }
            Operation::Write(write) => {
                let address = write.address | context.mask.one_mask;
                write_to_memory(&mut context, &write, 0, address);
            }
        }
    }

    context
}

fn write_to_memory(context: &mut ExecutionContext, write: &WriteOperation, index: usize, address: u64) {
    // Recursively dive into the floating mask, generating all permutations
    if index == 36 {
        // Write
        context.memory.insert(address, write.value);
    }
    else {
        // If this is a floating bit, generate all possibilities, else pass to the next bit
        if context.mask.floating_mask[index] {
            // Force 0
            write_to_memory(context, write, index + 1, address & !(1 << (35 - index)));
            // Force 1
            write_to_memory(context, write, index + 1, address | 1 << (35 - index));
        }
        else {
            write_to_memory(context, write, index + 1, address);
        }
    }

}

pub fn sum_of_all_mem_values_part_2(program: &Program) -> u64 {
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
        let content = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
".to_string();


        let program = parse_string(content);
        let result = sum_of_all_mem_values_part_2(&program);
        assert_eq!(result, 208);
    }
}