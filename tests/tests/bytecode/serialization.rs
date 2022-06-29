use crate::common::*;
use watchmaker_vm::*;

// Test serialization with some manually calculated bytecode values.

#[test]
fn serialize_nop() {
    let instruction = Instruction::NOP();

    let actual: u64 = watchmaker_vm::serialize(&instruction);
    let expected = 0x0000_0000_0000_0000;

    check_equal_u64(actual, expected);
}

#[test]
fn serialize_hlt() {
    let instruction = Instruction::HLT();

    let actual: u64 = watchmaker_vm::serialize(&instruction);
    let expected = 0x0200_0000_0000_0000;

    check_equal_u64(actual, expected);
}

#[test]
fn serialize_syn() {
    let instruction = Instruction::SYN();

    let actual: u64 = watchmaker_vm::serialize(&instruction);
    let expected = 0x0400_0000_0000_0000;

    check_equal_u64(actual, expected);
}

// #[test]
// fn serialize_iimov_direct_small() {
//     let instruction = Instruction::IIMOV(
//         LeftInteger::State(1, Mode::Direct),
//         RightInteger::State(2, Mode::Direct),
//     );
//
//     let actual: u64 = watchmaker_vm::serialize(&instruction);
//     let expected: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_DIRECT, MODE_OFFSET) | make_u16(1, INDEX_OFFSET),
//         make_u16(MODE_VALUE_DIRECT, MODE_OFFSET) | make_u16(2, INDEX_OFFSET),
//         0,
//     );
//
//     check_equal_u64(actual, expected);
// }
//
// #[test]
// fn serialize_iimov_indirect_small() {
//     let instruction = Instruction::IIMOV(
//         LeftInteger::State(1, Mode::Indirect),
//         RightInteger::State(2, Mode::Indirect),
//     );
//
//     let actual: u64 = watchmaker_vm::serialize(&instruction);
//     let expected: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(1, INDEX_OFFSET),
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(2, INDEX_OFFSET),
//         0,
//     );
//
//     check_equal_u64(actual, expected);
// }
//
// #[test]
// fn serialize_iimov_direct_max() {
//     let instruction = Instruction::IIMOV(
//         LeftInteger::State(INDEX_MAX, Mode::Direct),
//         RightInteger::State(INDEX_MAX, Mode::Direct),
//     );
//
//     let actual: u64 = watchmaker_vm::serialize(&instruction);
//     let expected: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_DIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         make_u16(MODE_VALUE_DIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         0,
//     );
//
//     check_equal_u64(actual, expected);
// }
//
// #[test]
// fn serialize_iimov_indirect_max() {
//     let instruction = Instruction::IIMOV(
//         LeftInteger::State(INDEX_MAX, Mode::Indirect),
//         RightInteger::State(INDEX_MAX, Mode::Indirect),
//     );
//
//     let actual: u64 = watchmaker_vm::serialize(&instruction);
//     let expected: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         0,
//     );
//
//     check_equal_u64(actual, expected);
// }
