use tests::create_factorial_virtual_machine;
use watchmaker_vm::*;

// Ad hoc example of a simple program.
pub fn main() {
    factorial(20);
}

pub fn factorial(n: u16) {
    let mut vm = create_factorial_virtual_machine();
    vm.iinput()[0] = n as i64;

    while vm.next_instruction() != &Instruction::HLT() {
        vm.run(1);
        // eprintln!(
        //     "position:{:?}, count:{:?}",
        //     vm.next_instruction_position(),
        //     vm.count()
        // );
        // eprintln!("memory: {:?}", vm.snapshot());
    }

    let result = vm.ioutput()[0];
    println!("factorial of {:?} is {:?}", vm.iinput()[0], result);
}
