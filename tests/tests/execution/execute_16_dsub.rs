// Test creation and execution of the virtual machine.

use crate::common::{check_equal_f64, make_test_architecture};
use watchmaker_vm::{Instruction, LeftDouble, Mode, RightDouble, VirtualMachine};

#[test]
fn dsub_can_subtract_inputs_and_store_result_in_output() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::DSUB(
            LeftDouble::Input(0, Mode::Direct),
            LeftDouble::Input(1, Mode::Direct),
            RightDouble::Output(2, Mode::Direct),
        )],
    );
    vm.dinput()[0] = 10.0;
    vm.dinput()[1] = 3.0;

    vm.run(1);

    let actual = vm.doutput()[2];
    check_equal_f64(actual, 7.0, 0.0);
}
