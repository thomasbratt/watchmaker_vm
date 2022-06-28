use crate::common::check_equal_instruction;
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn syn_roundtrips() {
    let expected = Instruction::SYN();

    let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

    check_equal_instruction(actual, expected);
}
