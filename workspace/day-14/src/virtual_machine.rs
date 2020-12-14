use std::collections::HashMap;

/// Represent a Mask operation, containing the map (overriden bit offset, value)
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct MaskOperation {
    pub zero_mask : u64,
    pub one_mask : u64,
    pub floating_mask : [bool; 36],
}

impl Default for MaskOperation {
    fn default() -> MaskOperation {
        MaskOperation {
            zero_mask: 0,
            one_mask: 0,
            floating_mask: [false; 36],
        }
    }
}

/// Represent a write operation of value at address
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct WriteOperation {
    pub address: u64,
    pub value: u64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Operation {
    Mask(MaskOperation),
    Write(WriteOperation),
}

pub type Program = Vec<Operation>;

#[derive(Debug, Default)]
pub struct ExecutionContext {
    pub mask: MaskOperation,
    pub memory: HashMap<u64, u64>
}