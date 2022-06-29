use crate::common::{all_code_offsets, all_ldoubles, check_equal_instruction};
use watchmaker_vm::*;

// Test instruction serialization and deserialization.
#[test]
fn djeq_roundtrips_all_operands() {
    for ldouble1 in all_ldoubles() {
        for ldouble2 in all_ldoubles() {
            for code_offset in all_code_offsets() {
                let expected = Instruction::DJEQ(ldouble1.clone(), ldouble2.clone(), code_offset);

                let actual: Instruction =
                    watchmaker_vm::deserialize(watchmaker_vm::serialize(&expected));

                check_equal_instruction(actual, expected);
            }
        }
    }
}
