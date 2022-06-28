use crate::common::*;
use clockwork::*;

#[test]
fn dimov_can_convert_from_input_to_output() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::DIMOV(
            LeftDouble::Input(0, Mode::Direct),
            RightInteger::Output(1, Mode::Direct),
        )],
    );
    vm.dinput()[0] = 42.12;

    vm.run(2);

    let actual = vm.ioutput()[1];
    check_equal_i64(actual, 42);
}

#[test]
fn dimov_succeeds_when_greater_than_max_u64() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![Instruction::DIMOV(
            LeftDouble::Input(0, Mode::Direct),
            RightInteger::Output(1, Mode::Direct),
        )],
    );
    vm.dinput()[0] = (u64::MAX as f64) * 2.0;

    vm.run(2);

    let actual = vm.ioutput()[1];
    assert!(actual > 0);
}
