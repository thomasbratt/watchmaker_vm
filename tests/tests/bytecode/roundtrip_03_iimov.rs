use crate::common::{all_lints, all_rints, check_equal_instruction};
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn iimov_roundtrips_all_operands() {
    for lint in all_lints() {
        for rint in all_rints() {
            let expected = Instruction::IIMOV(lint.clone(), rint.clone());

            let actual: Instruction =
                watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
