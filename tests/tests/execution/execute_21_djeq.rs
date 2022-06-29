// Test creation and execution of the virtual machine.

use crate::common::make_test_architecture;
use watchmaker_vm::{CodeOffset, Instruction, LeftDouble, Mode, VirtualMachine};

#[test]
fn djeq_can_jump_forward_on_match() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJEQ(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: 2 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 5.0;
    vm.dinput()[1] = 5.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 4);
}

#[test]
fn djeq_can_jump_backwards_on_match() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJEQ(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: -2 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 5.0;
    vm.dinput()[1] = 5.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 0);
}

#[test]
fn djeq_ignores_jump_zero() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJEQ(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: 0 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 5.0;
    vm.dinput()[1] = 5.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 3);
}

#[test]
fn djeq_ignores_not_equal() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJEQ(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: 0 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 1.0;
    vm.dinput()[1] = 100.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 3);
}
