// Test creation and execution of the virtual machine.

use crate::common::{check_equal_i64, make_test_architecture};
use clockwork::{Instruction, LeftInteger, Mode, RightInteger, VirtualMachine};

#[test]
fn iadd_can_add_inputs_and_store_result_in_output() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::IADD(
            LeftInteger::Input(0, Mode::Direct),
            LeftInteger::Input(1, Mode::Direct),
            RightInteger::Output(2, Mode::Direct),
        )],
    );
    vm.iinput()[0] = 39;
    vm.iinput()[1] = 3;

    vm.run(1);

    let actual = vm.ioutput()[2];
    check_equal_i64(actual, 42);
}
