use crate::common::{all_ldoubles, all_rdoubles, check_equal_instruction};
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn ddmov_roundtrips_all_operands() {
    for ldouble in all_ldoubles() {
        for rdouble in all_rdoubles() {
            let expected = Instruction::DDMOV(ldouble.clone(), rdouble.clone());

            let actual: Instruction =
                watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
