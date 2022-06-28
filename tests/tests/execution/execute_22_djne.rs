// Test creation and execution of the virtual machine.

use crate::common::make_test_architecture;
use clockwork::{CodeOffset, Instruction, LeftDouble, Mode, VirtualMachine};

#[test]
fn djne_can_jump_forward_on_match() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJNE(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: 2 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 5.0;
    vm.dinput()[1] = 6.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 4);
}

#[test]
fn djne_can_jump_backwards_on_match() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJNE(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: -2 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 5.0;
    vm.dinput()[1] = 6.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 0);
}

#[test]
fn djne_ignores_jump_zero() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJNE(
                LeftDouble::Input(0, Mode::Direct),
                LeftDouble::Input(1, Mode::Direct),
                CodeOffset { offset: 0 },
            ),
            Instruction::NOP(),
            Instruction::NOP(),
        ],
    );
    vm.dinput()[0] = 5.0;
    vm.dinput()[1] = 6.0;

    vm.run(3);

    let actual = vm.next_instruction_position();
    assert_eq!(actual, 3);
}

#[test]
fn djne_ignores_equals() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::NOP(),
            Instruction::NOP(),
            Instruction::DJNE(
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
