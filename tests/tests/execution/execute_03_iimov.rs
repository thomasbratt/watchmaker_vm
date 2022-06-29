use crate::common::*;
use watchmaker_vm::*;

#[test]
fn iimov_can_copy_from_input_to_output_via_state() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::IIMOV(
                LeftInteger::Input(0, Mode::Direct),
                RightInteger::State(1, Mode::Direct),
            ),
            Instruction::IIMOV(
                LeftInteger::State(1, Mode::Direct),
                RightInteger::Output(2, Mode::Direct),
            ),
        ],
    );
    vm.iinput()[0] = 42;

    vm.run(2);

    let actual = vm.ioutput()[2];
    check_equal_i64(actual, 42);
}
