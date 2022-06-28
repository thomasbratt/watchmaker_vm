use crate::common::{all_ldoubles, all_rints, check_equal_instruction};
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn dimov_roundtrips_all_operands() {
    for ldouble in all_ldoubles() {
        for rint in all_rints() {
            let expected = Instruction::DIMOV(ldouble.clone(), rint.clone());

            let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
