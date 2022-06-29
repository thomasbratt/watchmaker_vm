use crate::common::{all_lints, all_rints, check_equal_instruction};
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn isub_roundtrips_all_operands() {
    for lint1 in all_lints() {
        for lint2 in all_lints() {
            for rint in all_rints() {
                let expected = Instruction::ISUB(lint1.clone(), lint2.clone(), rint.clone());

                let actual: Instruction =
                    watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

                check_equal_instruction(actual, expected);
            }
        }
    }
}
