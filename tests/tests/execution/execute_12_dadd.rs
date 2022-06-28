// Test creation and execution of the virtual machine.

use crate::common::{check_equal_f64, make_test_architecture};
use clockwork::{Instruction, LeftDouble, Mode, RightDouble, VirtualMachine};

#[test]
fn dadd_can_add_inputs_and_store_result_in_output() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::DADD(
            LeftDouble::Input(0, Mode::Direct),
            LeftDouble::Input(1, Mode::Direct),
            RightDouble::Output(2, Mode::Direct),
        )],
    );
    vm.dinput()[0] = 3.123;
    vm.dinput()[1] = 7.456;

    vm.run(1);

    let actual = vm.doutput()[2];
    check_equal_f64(actual, 10.579, 0.0);
}
