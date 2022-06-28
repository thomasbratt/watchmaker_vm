use crate::common::{count_zeroes, make_test_architecture};
use clockwork::{Instruction, VirtualMachine};

#[test]
fn hlt_does_not_affect_memory() {
    let mut vm = VirtualMachine::new(&make_test_architecture(), vec![Instruction::HLT()]);
    vm.iinput()[0] = 42;
    vm.run(1);
    let memory = vm.snapshot();

    assert_eq!(count_zeroes(&memory), memory.count_elements() - 1);
}