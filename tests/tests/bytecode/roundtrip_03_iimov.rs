use crate::common::{all_lints, all_rints, check_equal_instruction};
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn iimov_roundtrips_all_operands() {
    for lint in all_lints() {
        for rint in all_rints() {
            let expected = Instruction::IIMOV(lint.clone(), rint.clone());

            let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
