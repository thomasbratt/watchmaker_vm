use crate::common::check_equal_instruction;
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn hlt_roundtrips() {
    let expected = Instruction::HLT();

    let actual: Instruction = watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

    check_equal_instruction(actual, expected);
}
