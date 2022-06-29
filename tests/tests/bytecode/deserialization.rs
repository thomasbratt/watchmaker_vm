use crate::common::*;
use watchmaker_vm::*;

// Test deserialization with some manually calculated bytecode values.

#[test]
fn deserialize_nop() {
    let actual: Instruction = deserialize(0x0000_0000_0000_0000);
    let expected: Instruction = Instruction::NOP();

    check_equal_instruction(actual, expected);
}

#[test]
fn deserialize_hlt() {
    let actual: Instruction = deserialize(0x0200_0000_0000_0000);
    let expected: Instruction = Instruction::HLT();

    check_equal_instruction(actual, expected);
}

#[test]
fn deserialize_syn() {
    let actual: Instruction = deserialize(0x0400_0000_0000_0000);
    let expected: Instruction = Instruction::SYN();

    check_equal_instruction(actual, expected);
}

#[test]
fn deserialize_iimov_direct_small() {
    let expected = Instruction::IIMOV(
        LeftInteger::State(1, Mode::Direct),
        RightInteger::State(2, Mode::Direct),
    );
    // let raw = watchmaker_vm::serialize(&expected);
    // eprint!("{}", BytecodeDisplay::from(raw));
    let actual: Instruction = watchmaker_vm::deserialize(0x0740_0068_0010_0000);

    check_equal_instruction(actual, expected);
}

// #[test]
// fn deserialize_iimov_indirect_small() {
//     let raw: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(1, INDEX_OFFSET),
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(2, INDEX_OFFSET),
//         0,
//     );
//
//     let actual: Instruction = watchmaker_vm::deserialize(raw);
//     let expected = Instruction::IIMOV(
//         LeftInteger::State(1, Mode::Indirect),
//         RightInteger::State(2, Mode::Indirect),
//     );
//
//     check_equal_instruction(actual, expected);
// }
//
// #[test]
// fn deserialize_iimov_direct_max() {
//     let raw: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_DIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         make_u16(MODE_VALUE_DIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         0,
//     );
//
//     let actual: Instruction = watchmaker_vm::deserialize(raw);
//     let expected = Instruction::IIMOV(
//         LeftInteger::State(INDEX_MAX, Mode::Direct),
//         RightInteger::State(INDEX_MAX, Mode::Direct),
//     );
//
//     check_equal_instruction(actual, expected);
// }
//
// #[test]
// fn deserialize_iimov_indirect_max() {
//     let raw: u64 = make_quad(
//         3,
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         make_u16(MODE_VALUE_INDIRECT, MODE_OFFSET) | make_u16(INDEX_MAX, INDEX_OFFSET),
//         0,
//     );
//
//     let actual: Instruction = watchmaker_vm::deserialize(raw);
//     let expected = Instruction::IIMOV(
//         LeftInteger::State(INDEX_MAX, Mode::Indirect),
//         RightInteger::State(INDEX_MAX, Mode::Indirect),
//     );
//
//     check_equal_instruction(actual, expected);
// }
