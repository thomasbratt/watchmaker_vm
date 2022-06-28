use crate::common::{all_lints, all_rdoubles, check_equal_instruction};
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn idmov_roundtrips_all_operands() {
    for lint in all_lints() {
        for rdouble in all_rdoubles() {
            let expected = Instruction::IDMOV(lint.clone(), rdouble.clone());

            let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

            check_equal_instruction(actual, expected);
        }
    }
}
