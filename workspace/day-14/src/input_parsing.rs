use std::fs;
use crate::virtual_machine::{Program, MaskOperation, WriteOperation, Operation};

pub fn parse_text_file(file_name: String) -> Program {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Program {
    content
        .split("\n")
        .filter(|str| !str.is_empty())
        .map(|instruction_string| {
            // Parse a mask operation
            if instruction_string.starts_with("mask") {
                let mut mask : MaskOperation = Default::default();
                let bit_mask_string = instruction_string.split_at(7).1;
                let mut i = 36;
                for c in bit_mask_string.chars() {
                    i -= 1;
                    match c {
                        '0' => { mask.zero_mask = mask.zero_mask | (1 << i); },
                        '1' => { mask.one_mask = mask.one_mask | (1 << i); },
                        'X' => { mask.floating_mask[35 - i] = true; },
                        _ => {}
                    }
                }
                Operation::Mask(mask)
            }
            else {
                let mut instruction_parts_string = instruction_string.split_whitespace();
                let mem_address_str = instruction_parts_string.nth(0).unwrap();
                let address : u64 = mem_address_str[4..mem_address_str.len() - 1].parse().expect("Unable to parse mem address");
                let value : u64 = instruction_parts_string.into_iter().nth_back(0).unwrap().parse().expect("Unable to parse value");

                Operation::Write(WriteOperation{ address, value })
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
".to_string();

        let program = parse_string(content);

        assert_eq!(program.len(), 4);

        assert_eq!(program[0], Operation::Mask(
            MaskOperation {
                zero_mask: 0b10,
                one_mask: 0b1000000,
                floating_mask: [true,true,true,true,true,true,true,true,
                    true,true,true,true,true,true,true,true,
                    true,true,true,true,true,true,true,true,
                    true,true,true,true,true,false,true,true,
                    true,true,false,true]
            })
        );
        assert_eq!(program[1], Operation::Write(WriteOperation { address: 8, value: 11 }));
        assert_eq!(program[2], Operation::Write(WriteOperation { address: 7, value: 101 }));
        assert_eq!(program[3], Operation::Write(WriteOperation { address: 8, value: 0 }));
    }
}