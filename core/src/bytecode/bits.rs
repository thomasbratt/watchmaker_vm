pub const OFFSET_OP_CODE: usize = 57;
pub const OFFSET_OPERAND_1: usize = 38;
pub const OFFSET_OPERAND_2: usize = 19;
pub const OFFSET_OPERAND_3: usize = 0;

pub const OFFSET_IS_ADDRESS: usize = 18;
pub const OFFSET_IS_INDIRECT: usize = 17;
pub const OFFSET_IS_STATE: usize = 16;
pub const OFFSET_FLAGS: usize = OFFSET_IS_STATE;

pub(crate) const MASK_OP_CODE: u64 = 0x0000_0000_0000_007F;
pub(crate) const MASK_OPERAND: u64 = 0x0000_0000_0007_FFFF;
pub(crate) const MASK_VALUE: u64 = 0x0000_0000_0000_FFFF;
pub(crate) const MASK_IS_ADDRESS: u64 = 0x0000_0000_0004_0000;
pub(crate) const MASK_IS_INDIRECT: u64 = 0x0000_0000_0002_0000;
pub(crate) const MASK_IS_STATE: u64 = 0x0000_0000_0001_0000;

pub const LENGTH_OP_CODE: usize = 7;
pub const LENGTH_FLAGS: usize = 3;
pub const LENGTH_VALUE: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub struct InstructionFields {
    pub op_code: u8,
    pub operand1: OperandFields,
    pub operand2: OperandFields,
    pub operand3: OperandFields,
    pub raw: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OperandFields {
    pub is_address: bool,
    pub is_indirect: bool,
    pub is_state: bool,
    pub value: u16,
    pub raw: u64,
}

/// Extract raw bit fields from a 64 bit instruction.
///
///  7 bit opcode     - the instruction field
///  19 bit operand 1 - as tuple (is_address:1-bit, is_indirect:1-bit, is_state:1-bit, value:16-bit)
///  19 bit operand 2 - as tuple (is_address:1-bit, is_indirect:1-bit, is_state:1-bit, value:16-bit)
///  19 bit operand 3 - as tuple (is_address:1-bit, is_indirect:1-bit, is_state:1-bit, value:16-bit)
///
pub fn extract_fields(raw: u64) -> InstructionFields {
    InstructionFields {
        op_code: extract_op_code(raw),
        operand1: extract_operand1(raw),
        operand2: extract_operand2(raw),
        operand3: extract_operand3(raw),
        raw,
    }
}

fn extract_op_code(raw: u64) -> u8 {
    ((raw >> OFFSET_OP_CODE) & MASK_OP_CODE) as u8
}

fn extract_operand1(raw: u64) -> OperandFields {
    extract_operand(raw, OFFSET_OPERAND_1)
}

fn extract_operand2(raw: u64) -> OperandFields {
    extract_operand(raw, OFFSET_OPERAND_2)
}

fn extract_operand3(raw: u64) -> OperandFields {
    extract_operand(raw, OFFSET_OPERAND_3)
}

fn extract_operand(raw: u64, base: usize) -> OperandFields {
    let o = (raw >> base) & MASK_OPERAND;
    OperandFields {
        is_address: o & MASK_IS_ADDRESS == MASK_IS_ADDRESS,
        is_indirect: o & MASK_IS_INDIRECT == MASK_IS_INDIRECT,
        is_state: o & MASK_IS_STATE == MASK_IS_STATE,
        value: (o & MASK_VALUE) as u16,
        raw: o,
    }
}
