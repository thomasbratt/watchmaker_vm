# watchmaker_vm

A virtual machine for use with [genetic algorithms](https://en.wikipedia.org/wiki/Genetic_algorithm).

The virtual machine has an instruction set that is much simpler than real world processors. 
There are no registers and all operands are memory locations.
This has the benefits that instructions are easier to interpret with less need to
evolve needless complexity around register usage.

[![CircleCI](https://circleci.com/gh/thomasbratt/watchmaker_vm/tree/main.svg?style=svg)](https://circleci.com/gh/thomasbratt/watchmaker_vm/tree/main)

## Features

* Suitable for Genetic Algorithms.  Any random sequence of bytes is a valid program.
* Simple orthogonal instruction format. Each instruction is a 64 bit value, including operands.
* Fairly efficient, with a lower branching factor per instruction than architectures that
include registers or require processing expression trees.
* Utilities for dumping instructions and serializing to and from 64-bit values.
* Flexible memory mapped I/O.

## Usage

* Add the following line to your `Cargo.toml` file:
```yaml
[dependencies]
watchmaker_vm = "1.0.1"
```
* Latest published version: https://crates.io/crates/watchmaker_vm

## Examples

The following creates a virtual machine that executes a factorial function.
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

The following shows how to create a random program that can be supplied to a virtual machine instance.

```rust
        let raw: Vec<u64> = (0..GENOME_SIZE)
            .into_iter()
            .map(|_| rand::thread_rng().next_u64())
            .collect();
        let instructions: Vec<Instruction> = raw.into_iter()
            .map(watchmaker_vm::deserialize)
            .collect();
```

## Bytecode

```code

<7 bits>|<---- 19 bits ---->|<---- 19 bits ---->|<---- 19 bits ---->|
+-------+-------------------+-------------------+-------------------+
|opcode |    operand 1      |    operand 2      |    operand 3      |
+-------+-------------------+-------------------+-------------------+

19 bit operand:
   1-bit is_address
   1-bit is_indirect
   1-bit is_state
  16-bit value

value is one of:
  16-bit signed integer instruction offset
  16-bit integer constant
  16-bit floating point constant
  16 bit integer memory address
  16 bit integer indirect memory address (state register at this location holds actual address)

```

## Instructions

```code
Instructions
--------------------------------------------------------------------------------

All instructions given in the format:

    mnemonic [operand1] [operand2] [->] [operand3]  [# comment]

# no operands required
NOP                                 # no operation
HLT                                 # sync and stop
SYN                                 # sync input and output register arrays with outside environment

IIMOV lint rint                     # Copy the value of lint to rint.
IDMOV lint rdouble                  # Copy the value of lint to rdouble, converting type.
DIMOV ldouble rint                  # Copy the value of ldouble to rint, converting type.
DDMOV ldouble rdouble               # Copy the value of ldouble to rdouble.

IADD lint lint rint                 # lint + lint           -> rint
IDIV lint lint rint                 # lint / lint           -> rint, divide by zero results in zero
IMOD lint lint rint                 # lint % lint           -> rint
IMUL lint lint rint                 # lint * lint           -> rint
ISUB lint lint rint                 # lint - lint           -> rint

DADD ldouble ldouble rdouble        # lint + lint           -> rint
DDIV ldouble ldouble rdouble        # lint / lint           -> rint, divide by zero results in zero
DMOD ldouble ldouble rdouble        # lint % lint           -> rint
DMUL ldouble ldouble rdouble        # lint * lint           -> rint
DSUB ldouble ldouble rdouble        # lint - lint           -> rint

# caddr must be a relative instruction offset
IJEQ lint lint caddr                # if operand1 == operand2, jump to caddr
IJNE lint lint caddr                # if operand1 != operand2, jump to caddr
IJGT lint lint caddr                # if operand1 > operand2, jump to caddr
IJLT lint lint caddr                # if operand1 < operand2, jump to caddr

# caddr must be a relative instruction offset
DJEQ ldouble ldouble caddr          # if operand1 == operand2, jump to caddr
DJNE ldouble ldouble caddr          # if operand1 != operand2, jump to caddr
DJGT ldouble ldouble caddr          # if operand1 > operand2, jump to caddr
DJLT ldouble ldouble caddr          # if operand1 < operand2, jump to caddr

```

## Architecture

```code
II: linear array of integer input registers
IS: linear array of integer state registers
IO: linear array of integer output registers

DI: linear array of double precision floating point input registers
DS: linear array of double precision floating point state registers
DO: linear array of double precision floating point output registers

C: 64 bit instructions code stored in a linear array
```

All registers are initialized to zero at start up.
The input registers can be written to at any time.
The output registers can be read at any time.

Operand addresses are evaluated modulo the size of the register array they refer to, to keep them within the
allocated memory for that array.

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
