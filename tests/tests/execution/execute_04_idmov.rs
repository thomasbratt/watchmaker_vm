use crate::common::*;
use watchmaker_vm::*;

#[test]
fn idmov_can_convert_from_input_to_output() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::IDMOV(
            LeftInteger::Input(0, Mode::Direct),
            RightDouble::Output(1, Mode::Direct),
        )],
    );
    vm.iinput()[0] = 42;

    vm.run(2);

    let actual = vm.doutput()[1];
    check_equal_f64(actual, 42.0, 0.0);
}
