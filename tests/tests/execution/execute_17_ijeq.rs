// Test creation and execution of the virtual machine.

use crate::common::make_test_architecture;
use clockwork::{CodeOffset, Instruction, LeftInteger, Mode, VirtualMachine};

#[test]
fn ijeq_can_jump_forward_on_match() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::IJEQ(
                LeftInteger::Input(0, Mode::Direct),
                LeftInteger::Input(1, Mode::Direct),
                CodeOffset { offset: 2 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.iinput()[0] = 5;
    vm.iinput()[1] = 5;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 4);
}

#[test]
fn ijeq_can_jump_backwards_on_match() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::IJEQ(
                LeftInteger::Input(0, Mode::Direct),
                LeftInteger::Input(1, Mode::Direct),
                CodeOffset { offset: -2 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.iinput()[0] = 5;
    vm.iinput()[1] = 5;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 0);
}

#[test]
fn ijeq_ignores_jump_zero() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::IJEQ(
                LeftInteger::Input(0, Mode::Direct),
                LeftInteger::Input(1, Mode::Direct),
                CodeOffset { offset: 0 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.iinput()[0] = 5;
    vm.iinput()[1] = 5;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 3);
}

#[test]
fn ijeq_ignores_not_equal() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::IJEQ(
                LeftInteger::Input(0, Mode::Direct),
                LeftInteger::Input(1, Mode::Direct),
                CodeOffset { offset: 0 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.iinput()[0] = 1;
    vm.iinput()[1] = 100;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 3);
}
