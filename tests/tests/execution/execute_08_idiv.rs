// Test creation and execution of the virtual machine.

use crate::common::{check_equal_i64, make_test_architecture};
use clockwork::{Instruction, LeftInteger, Mode, RightInteger, VirtualMachine};

#[test]
fn idiv_can_divide_input_values_and_store_result_in_output() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::IDIV(
            LeftInteger::Input(0, Mode::Direct),
            LeftInteger::Input(1, Mode::Direct),
            RightInteger::Output(2, Mode::Direct),
        )],
    );
    vm.iinput()[0] = 15;
    vm.iinput()[1] = 3;

    vm.run(1);

    let actual = vm.ioutput()[2];
    check_equal_i64(actual, 5);
}

#[test]
fn idiv_can_divide_negative_values() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::IDIV(
            LeftInteger::Input(0, Mode::Direct),
            LeftInteger::Input(1, Mode::Direct),
            RightInteger::Output(2, Mode::Direct),
        )],
    );
    vm.iinput()[0] = 10;
    vm.iinput()[1] = -5;

    vm.run(1);

    let actual = vm.ioutput()[2];
    check_equal_i64(actual, -2);
}

#[test]
fn idiv_can_handle_divide_by_zero() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::IDIV(
            LeftInteger::Input(0, Mode::Direct),
            LeftInteger::Input(1, Mode::Direct),
            RightInteger::Output(2, Mode::Direct),
        )],
    );
    vm.iinput()[0] = 15;
    vm.iinput()[1] = 0;

    vm.run(1);

    let actual = vm.ioutput()[2];
    check_equal_i64(actual, 0);
}
