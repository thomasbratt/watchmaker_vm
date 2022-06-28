use crate::common::{all_code_offsets, all_lints, check_equal_instruction};
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn ijlt_roundtrips_all_operands() {
    for lint1 in all_lints() {
        for lint2 in all_lints() {
            for code_offset in all_code_offsets() {
                let expected = Instruction::IJLT(lint1.clone(), lint2.clone(), code_offset);

                let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

                check_equal_instruction(actual, expected);
            }
        }
    }
}
