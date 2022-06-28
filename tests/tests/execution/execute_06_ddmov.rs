use crate::common::*;
use clockwork::*;

#[test]
fn ddmov_can_copy_from_input_to_output_via_state() {
    let mut vm = VirtualMachine::new(
        &make_test_architecture(),
        vec![
            Instruction::DDMOV(
                LeftDouble::Input(0, Mode::Direct),
                RightDouble::State(1, Mode::Direct),
            ),
            Instruction::DDMOV(
                LeftDouble::State(1, Mode::Direct),
                RightDouble::Output(2, Mode::Direct),
            ),
        ],
    );
    let expected = 42.1234;
    vm.dinput()[0] = expected;

    vm.run(2);

    let actual = vm.doutput()[2];
    check_equal_f64(actual, expected, 0.0);
}
