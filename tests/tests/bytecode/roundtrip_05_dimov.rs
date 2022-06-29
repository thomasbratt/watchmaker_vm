use crate::common::{all_ldoubles, all_rints, check_equal_instruction};
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn dimov_roundtrips_all_operands() {
    for ldouble in all_ldoubles() {
        for rint in all_rints() {
            let expected = Instruction::DIMOV(ldouble.clone(), rint.clone());

            let actual: Instruction =
                watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
