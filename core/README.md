# watchmaker_vm

A virtual machine for use with genetic algorithms.

The virtual machine has an instruction set that is much simpler than real world processors. 
There are no registers and all operands are memory locations.
This has the benefits that instructions are easier to interpret with less need to
evolve needless complexity around register usage.

[![CircleCI](https://circleci.com/gh/thomasbratt/watchmaker_vm/tree/main.svg?style=svg)](https://circleci.com/gh/thomasbratt/watchmaker_vm/tree/main)

## Features

* Suitable for Genetic Algorithms.  Any random sequence of bytes is a valid program.
* Simple instruction format. Each instruction is a 64 bit value, including operands.
* Fairly efficient, with a lower branching factor per instruction than architectures that
include registers or require processing expression trees.
* Utilities for dumping instructions.
* Flexible memory mapped I/O.

## Usage

* Install Rust using rustup <https://rustup.rs/>
* Clone the repository (see below)
* Run `cargo test` or `cargo build`

## Example

The following creates an example that executes a factorial function.
```rust
    let vm = VirtualMachine::new(
        &ArchitectureBuilder::default()
            .iinput(1)
            .istate(2)
            .ioutput(1)
            .dinput(1)
            .dstate(1)
            .doutput(1)
            .build()
            .unwrap(),
        vec![
            Instruction::IIMOV(
                LeftInteger::Input(0, Mode::Direct),
                RightInteger::State(0, Mode::Direct),
            ),
            Instruction::IIMOV(
                LeftInteger::Input(0, Mode::Direct),
                RightInteger::State(1, Mode::Direct),
            ),
            Instruction::IJLT(
                LeftInteger::State(0, Mode::Direct),
                LeftInteger::Constant(2),
                CodeOffset { offset: 4 },
            ),
            Instruction::ISUB(
                LeftInteger::State(0, Mode::Direct),
                LeftInteger::Constant(1),
                RightInteger::State(0, Mode::Direct),
            ),
            Instruction::IMUL(
                LeftInteger::State(0, Mode::Direct),
                LeftInteger::State(1, Mode::Direct),
                RightInteger::State(1, Mode::Direct),
            ),
            Instruction::IJEQ(
                LeftInteger::Constant(0),
                LeftInteger::Constant(0),
                CodeOffset { offset: -3 },
            ),
            Instruction::IIMOV(
                LeftInteger::State(1, Mode::Direct),
                RightInteger::Output(0, Mode::Direct),
            ),
            Instruction::HLT(),
        ],
    );
    // Write the input to the first location in the integer typed input memory bank.
    vm.iinput()[0] = n as i64;
    
    // Execute the VM until the halt instruction is reached.
    while vm.next_instruction() != &Instruction::HLT() {
        vm.run(1);
    }
    
    // Read the result from the first location of the integer typed output memory bank.
    let result = vm.ioutput()[0];
    println!("factorial of {:?} is {:?}", vm.iinput()[0], result);
```
## Alternatives

The Genetic Algorithm is a very well known technique and as a result there are many alternative ways of representing and
executing evolved programs.

Some candidates:

* `Genetic Programming` Very well known and studied. Represent code as a tree of operators and operands.
More complicated to process.  Can introduce bias into subtrees.
Does not typically support memory mapped I/O.
<https://en.wikipedia.org/wiki/Genetic_programming#Program_representation>
* `MLeM` A Rust crate implementing a VM specifically for Genetic Algorithms.
  Supports memory mapped I/O.
  Similar to this crate but uses registers and supports use of a stack.
  Looks like it would use more branches per instruction.
  <https://crates.io/crates/mlem>
* `Slash/A` "A programming language and C++ library for (quantitative) linear genetic programming."
  Very minimal and worth looking at for that reason alone.
  Restrictive memory architecture but could be expanded.
  Does not easily support memory mapped I/O.
  Each instruction does little, so vulnerable to needless complexity (the 'Turing Tarpit').
  <https://github.com/arturadib/slash-a>
* `Turing Machine` The original Virtual Machine. Very well known and studied.
Each instruction does little, so vulnerable to needless complexity (the 'Turing Tarpit').
Can be difficult to interpret (see 'Brainfuck' <https://en.wikipedia.org/wiki/Brainfuck>)
<https://en.wikipedia.org/wiki/Turing_machine>

## License

MIT permissive license. See LICENSE for full license details.

## Source Code Repository

<https://github.com/thomasbratt/watchmaker_vm>

