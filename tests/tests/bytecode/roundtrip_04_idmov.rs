use crate::common::{all_lints, all_rdoubles, check_equal_instruction};
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn idmov_roundtrips_all_operands() {
    for lint in all_lints() {
        for rdouble in all_rdoubles() {
            let expected = Instruction::IDMOV(lint.clone(), rdouble.clone());

            let actual: Instruction =
                watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
