use crate::common::{all_ldoubles, all_rdoubles, check_equal_instruction};
use clockwork::*;

// Test instruction serialization and deserialization.
#[test]
fn dmod_roundtrips_all_operands() {
    for ldouble1 in all_ldoubles() {
        for ldouble2 in all_ldoubles() {
            for rdouble in all_rdoubles() {
                let expected =
                    Instruction::DMOD(ldouble1.clone(), ldouble2.clone(), rdouble.clone());

                let actual: Instruction = clockwork::deserialize(clockwork::serialize(&expected));

                check_equal_instruction(actual, expected);
            }
        }
    }
}
