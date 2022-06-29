use crate::common::check_equal_instruction;
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn nop_roundtrips() {
    let expected = Instruction::NOP();

    let actual: Instruction = watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

    check_equal_instruction(actual, expected);
}
