use crate::common::{all_lints, all_rints, check_equal_instruction};
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn imul_roundtrips_all_operands() {
    for lint1 in all_lints() {
        for lint2 in all_lints() {
            for rint in all_rints() {
                let expected = Instruction::IMUL(lint1.clone(), lint2.clone(), rint.clone());

                let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

                check_equal_instruction(actual, expected);
            }
        }
    }
}
