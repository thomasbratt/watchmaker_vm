use crate::bytecode::binary_field_display::BinaryFieldDisplay;
use crate::*;
use std::fmt;

/// Indented formatting for the fields of a raw u64 bytecode instruction.
#[derive(Clone, Debug, PartialEq)]
pub struct BytecodeDisplay {
    display: BinaryFieldDisplay,
}

impl BytecodeDisplay {
    /// Create a display object from raw u64 bytecode.
    pub fn from(raw: u64) -> Self {
        Self {
            display: BinaryFieldDisplay::from(raw),
        }
    }

    /// Display all fields.
    pub fn all(&self) -> String {
        self.display.all()
    }

    pub fn op_code(&self) -> String {
        self.display.at(OFFSET_OP_CODE, LENGTH_OP_CODE)
    }

    pub fn operand1_flags(&self) -> String {
        self.display
            .at(OFFSET_OPERAND_1 + OFFSET_FLAGS, LENGTH_FLAGS)
    }

    pub fn operand1_value(&self) -> String {
        self.display.at(OFFSET_OPERAND_1, LENGTH_VALUE)
    }

    pub fn operand2_flags(&self) -> String {
        self.display
            .at(OFFSET_OPERAND_2 + OFFSET_FLAGS, LENGTH_FLAGS)
    }

    pub fn operand2_value(&self) -> String {
        self.display.at(OFFSET_OPERAND_2, LENGTH_VALUE)
    }

    pub fn operand3_flags(&self) -> String {
        self.display
            .at(OFFSET_OPERAND_3 + OFFSET_FLAGS, LENGTH_FLAGS)
    }

    pub fn operand3_value(&self) -> String {
        self.display.at(OFFSET_OPERAND_3, LENGTH_VALUE)
    }
}

impl fmt::Display for BytecodeDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "deserialized   : {:?}", deserialize(self.display.raw()))?;
        writeln!(f, "hex            : {:016x}", self.display.raw())?;
        writeln!(f, "raw            : {}", self.all())?;
        writeln!(f, "op_code        : {}", self.op_code())?;
        writeln!(f, "operand 1 flags: {}", self.operand1_flags())?;
        writeln!(f, "          value: {}", self.operand1_value())?;
        writeln!(f, "operand 2 flags: {}", self.operand2_flags())?;
        writeln!(f, "          value: {}", self.operand2_value())?;
        writeln!(f, "operand 3 flags: {}", self.operand3_flags())?;
        writeln!(f, "          value: {}", self.operand3_value())
    }
}
