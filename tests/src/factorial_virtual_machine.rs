use clockwork::*;

pub fn create_factorial_virtual_machine() -> VirtualMachine {
    VirtualMachine::new(
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
    )
}
